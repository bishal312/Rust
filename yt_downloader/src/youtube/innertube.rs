use anyhow::{Context, Result, bail};
use reqwest::header::{COOKIE, HeaderValue};
use serde_json::{Value, json};

use super::{client::ClientProfile, models::PlayerResponse, video_id::VideoId};

const PLAYER_URL: &str = "https://www.youtube.com/youtubei/v1/player?prettyPrint=false";

// Returns the raw JSON first. looking real responses
// before deciding what our sturcts should contain.

pub async fn fetch_player_raw(
    http: &reqwest::Client,
    video_id: &VideoId,
    client: &ClientProfile,
    cookie: Option<&HeaderValue>,
) -> Result<Value> {
    let body = json!({
        "context": client.context,
        "videoId": video_id.as_str(),
        "contentCheckOk": true,
        "racyCheckOk": true,
    });

    let mut request = http
        .post(PLAYER_URL)
        .header("User-Agent", client.user_agent)
        .header("X-Youtube-Client-Name", client.client_id.to_string())
        .header("X-YouTube-Client-Version", client.client_version)
        .json(&body);
    if let Some(cookie) = cookie {
        request = request.header(COOKIE, cookie.clone());
    }

    request
        .send()
        .await
        .context("request to /player failed")?
        .error_for_status()
        .context("YouTube answered with an error status")?
        .json::<Value>()
        .await
        .context("response was not valid JSON")
}

pub async fn fetch_player_with_fallback<'a>(
    http: &reqwest::Client,
    video_id: &VideoId,
    profiles: &'a [ClientProfile],
    cookie: Option<&HeaderValue>,
) -> Result<(PlayerResponse, &'a ClientProfile)> {
    let mut failures: Vec<String> = Vec::new();

    for profile in profiles {
        match try_profile(http, video_id, profile, cookie).await {
            Ok(player) => return Ok((player, profile)),
            Err(e) => failures.push(format!("{}: {e:#}", profile.name)),
        }
    }
    bail!("no client profile worked:\n {}", failures.join("\n "))
}

async fn try_profile(
    http: &reqwest::Client,
    video_id: &VideoId,
    profile: &ClientProfile,
    cookie: Option<&HeaderValue>,
) -> Result<PlayerResponse> {
    let raw = fetch_player_raw(http, video_id, profile, cookie).await?;
    let player: PlayerResponse =
        serde_json::from_value(raw).context("unexpected response shape")?;

    if player.playability_status.status != "OK" {
        bail!(
            "{} ({})",
            player.playability_status.status,
            player
                .playability_status
                .reason
                .as_deref()
                .unwrap_or("no reason given")
        );
    }

    let has_direct_url = player.streaming_data.as_ref().is_some_and(|sd| {
        sd.formats
            .iter()
            .chain(sd.adaptive_formats.iter())
            .any(|f| f.url.is_some())
    });

    if !has_direct_url {
        bail!("status OK, but no format carries a direct url");
    }

    Ok(player)
}
