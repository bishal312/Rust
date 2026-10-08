use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};

use serde::Deserialize;
use tokio::net::windows::named_pipe::PipeEnd::Client;
use tower_http::services::ServeDir;

use crate::youtube::{
    client::ClientProfile, info::InfoResponse, innertube::fetch_player_with_fallback,
    video_id::parse_video_id,
};

type ApiError = (StatusCode, String);

#[derive(Clone)]
pub struct AppState {
    pub http: reqwest::Client,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(|| async { "ok" }))
        .route("/api/info", post(info))
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
    let id = parse_video_id(&req.url).map_err(|e| (StatusCode::BAD_REQUEST))?;
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
