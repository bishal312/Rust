use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use serde::Deserialize;
use serde_json::json;
use tower_http::services::ServeDir;

use crate::youtube::info::InfoResponse;
use crate::youtube::{
    client::ClientProfile, innertube::fetch_player_with_fallback, video_id::parse_video_id,
};
use crate::{download::download_pair, youtube::select::pick_formats};

type ApiError = (StatusCode, String);

#[derive(Clone)]
pub struct AppState {
    pub http: reqwest::Client,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/info", post(info))
        .route("/api/debug/download", post(debug_download))
        .fallback_service(ServeDir::new("static"))
        .with_state(state)
}

#[derive(Deserialize)]
struct InfoRequest {
    url: String,
}

async fn info(
    State(state): State<AppState>,
    Json(req): Json<InfoRequest>,
) -> Result<Json<InfoResponse>, ApiError> {
    let id = parse_video_id(&req.url).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let profiles = [ClientProfile::android_vr(), ClientProfile::ios()];
    let (player, used) = fetch_player_with_fallback(&state.http, &id, &profiles)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e:#}")))?;

    let info = InfoResponse::from_player(&player, used.name).ok_or((
        StatusCode::BAD_GATEWAY,
        "response had no video details or streaming data".to_string(),
    ))?;
    Ok(Json(info))
}

#[derive(Deserialize)]
struct DonwloadRequest {
    url: String,
    #[serde(default = "default_max_height")]
    max_height: u32,
}

fn default_max_height() -> u32 {
    1080
}

async fn debug_download(
    State(state): State<AppState>,
    Json(req): Json<DonwloadRequest>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let id = parse_video_id(&req.url).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let profiles = [ClientProfile::android_vr(), ClientProfile::ios()];
    let (player, used) = fetch_player_with_fallback(&state.http, &id, &profiles)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e:#}")))?;

    let streaming = player
        .streaming_data
        .as_ref()
        .ok_or((StatusCode::BAD_GATEWAY, "no streaming data".to_string()))?;

    let (video, audio) = pick_formats(&streaming.adaptive_formats, req.max_height).ok_or((
        StatusCode::UNPROCESSABLE_ENTITY,
        "no H.264 + AAC MP4 pair available".to_string(),
    ))?;

    let total = video.content_length().unwrap_or(0) + audio.content_length().unwrap_or(0);
    let progress = Arc::new(AtomicU64::new(0));

    // print progress to the terminal once per second while downloading.
    let ticker = {
        let progress = progress.clone();
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                let done = progress.load(Ordering::Relaxed);
                println!(
                    "progress: {:.1} / {:.1} MiB",
                    done as f64 / 1_048_576.0,
                    total as f64 / 1_048_576.0
                );
            }
        })
    };

    let dir = PathBuf::from("downloads").join(id.as_str());
    let result = download_pair(&state.http, video, audio, used.user_agent, &dir, progress).await;
    ticker.abort();

    let (video_path, audio_path) =
        result.map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e:#}")))?;

    Ok(Json(json!({
        "client": used.name,
        "video_itag": video.itag,
        "audio_itag": audio.itag,
        "video_file": video_path.display().to_string(),
        "audio_file": audio_path.display().to_string(),
    })))
}
