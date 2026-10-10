use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use axum::http;
use reqwest::StatusCode;
use reqwest::header::{RANGE, USER_AGENT};
use tokio::io::AsyncWriteExt;

use crate::youtube::{
    client::ClientProfile,
    innertube::fetch_player_with_fallback,
    models::{Format, PlayerResponse},
    select::pick_formats,
    video_id::VideoId,
};

const CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4 MiB per request
const MAX_ATTEMPTS: u32 = 3;

// Downloads the video and audio streams at the same time into `dir`.
// Both tasks add to the same shared `progress` counter (total bytes so far).
pub async fn download_pair(
    http: &reqwest::Client,
    video: &Format,
    audio: &Format,
    user_agent: &str,
    dir: &Path,
    progress: Arc<AtomicU64>,
) -> Result<(PathBuf, PathBuf)> {
    tokio::fs::create_dir_all(dir)
        .await
        .context("cannot create download directory")?;

    let vidoe_path = dir.join("vido.mp4");
    let audio_path = dir.join("audio.m4a");

    // Runs both downloads concurrently; if either fails, we get the error.
    tokio::try_join!(
        download_format(http, video, user_agent, &vidoe_path, progress.clone()),
        download_format(http, video, user_agent, &vidoe_path, progress.clone()),
    )?;

    Ok((vidoe_path, audio_path))
}

async fn download_format(
    http: &reqwest::Client,
    format: &Format,
    user_agent: &str,
    dest: &Path,
    progress: Arc<AtomicU64>,
) -> Result<()> {
    let url = format.url.as_deref().context("format has no direct url")?;
    let total = format
        .content_length()
        .context("format has no contentLength")?;

    let mut file = tokio::fs::File::create(dest)
        .await
        .with_context(|| format!("cannot create {}", dest.display()))?;

    let mut start = 0u64;
    while start < total {
        let end = (start + CHUNK_SIZE).min(total) - 1; // inclusive
        let bytes = fetch_chunk_with_retry(http, url, user_agent, start, end, &progress).await?;
        file.write_all(&bytes).await?;
        start = end + 1;
    }

    file.flush().await?;
    Ok(())
}

async fn fetch_chunk_with_retry(
    http: &reqwest::Client,
    url: &str,
    user_agent: &str,
    start: u64,
    end: u64,
    progress: &AtomicU64,
) -> Result<Vec<u8>> {
    let mut last_error = None;

    for attempt in 1..=MAX_ATTEMPTS {
        let mut counted = 0u64; // bytes this attempt added to `progress`
        match fetch_chunk(http, url, user_agent, start, end, progress, &mut counted).await {
            Ok(bytes) => return Ok(bytes),
            Err(e) => {
                progress.fetch_sub(counted, Ordering::Relaxed); // undo partial progress
                eprint!("chunk {start}-{end} failed (attempt {attempt}/{MAX_ATTEMPTS}): {e:#}");
                last_error = Some(e);
                tokio::time::sleep(Duration::from_millis(500 * attempt as u64)).await;
            }
        }
    }
    Err(last_error.expect("MAX_ATTEMPTS is at least 1"))
}

async fn fetch_chunk(
    http: &reqwest::Client,
    url: &str,
    user_agent: &str,
    start: u64,
    end: u64,
    progress: &AtomicU64,
    counted: &mut u64,
) -> Result<Vec<u8>> {
    let mut resp = http
        .get(url)
        .header(USER_AGENT, user_agent)
        .header(RANGE, format!("bytes={start}-{end}"))
        .send()
        .await
        .context("request failed")?;

    ensure!(
        resp.status() == StatusCode::PARTIAL_CONTENT,
        "expected 206 Partial Content, got {}",
        resp.status()
    );

    let expected = (end - start + 1) as usize;
    let mut buf = Vec::with_capacity(expected);

    while let Some(piece) = resp.chunk().await.context("connection dropped mid-chunk")? {
        buf.extend_from_slice(&piece);
        progress.fetch_add(piece.len() as u64, Ordering::Relaxed);
        *counted += piece.len() as u64;
    }

    ensure!(
        buf.len() == expected,
        "short read: got {} of {} bytes",
        buf.len(),
        expected
    );

    Ok(buf)
}

// asking for the last 1 KiB of a stream. If the server only serves the first
// part of a file to this client, this fails now instead of mid-download.

pub async fn probe_tail(http: &reqwest::Client, format: &Format, user_agent: &str) -> Result<()> {
    let url = format.url.as_deref().context("format has no direct url")?;
    let total = format
        .content_length()
        .context("format has no contentLength")?;
    ensure!(total > 0, "empty stream");

    let start = total.saturating_sub(1024);
    let end = total - 1;

    let resp = http
        .get(url)
        .header(USER_AGENT, user_agent)
        .header(RANGE, format!("bytes={start}-{end}"))
        .send()
        .await
        .context("probe request failed")?;

    ensure!(
        resp.status() == StatusCode::PARTIAL_CONTENT,
        "probe of bytes {start}-{end}: expected 206, got {}",
        resp.status()
    );

    Ok(())
}

// tries each client in order and returns the first one whose streams
// actually deliver the end of the file, not just the beginning.
pub async fn find_working_client<'a>(
    http: &reqwest::Client,
    id: &VideoId,
    profiles: &'a [ClientProfile],
    max_height: u32,
) -> Result<(PlayerResponse, &'a ClientProfile)> {
    let mut failures = Vec::new();

    for profile in profiles {
        match try_for_download(http, id, profile, max_height).await {
            Ok(player) => return Ok((player, profile)),
            Err(e) => {
                eprintln!("client {} rejected: {e:#}", profile.name);
                failures.push(format!("{}: {e:#}", profile.name));
            }
        }
    }
    bail!(
        "no client cloud deliver this video:\n {}",
        failures.join("\n  ")
    )
}

async fn try_for_download(
    http: &reqwest::Client,
    id: &VideoId,
    profile: &ClientProfile,
    max_height: u32,
) -> Result<PlayerResponse> {
    // reuse the single-profile logic by passing a one-element slice.
    let (player, _) = fetch_player_with_fallback(http, id, std::slice::from_ref(profile)).await?;

    {
        let streaming = player
            .streaming_data
            .as_ref()
            .context("no streaming data")?;
        let (video, audio) = pick_formats(&streaming.adaptive_formats, max_height)
            .context("no H.264 + AAC MP4 pair available")?;

        probe_tail(http, video, profile.user_agent)
            .await
            .context("video stream")?;
        probe_tail(http, audio, profile.user_agent)
            .await
            .context("audio stream")?;
    } // `streaming`, `video`, `audio`, stop borrowing `player` here

    Ok(player)
}
