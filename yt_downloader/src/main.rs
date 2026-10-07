mod youtube;
use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};
use serde::Deserialize;
use tower_http::services::ServeDir;
use youtube::{
    client::ClientProfile, innertube::fetch_player_raw, models::PlayerResponse,
    video_id::parse_video_id,
};

#[derive(Deserialize)]
struct PlayerQuery {
    url: String,
}

async fn debug_player(
    Query(q): Query<PlayerQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let id = parse_video_id(&q.url).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // A new client per request is wasteful;
    let http = reqwest::Client::new();

    let client = ClientProfile::android_vr();

    let raw = fetch_player_raw(&http, &id, &client)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e:#}")))?;

    let player: PlayerResponse = serde_json::from_value(raw.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    println!(
        "playability: {} ({:?})",
        player.playability_status.status, player.playability_status.reason
    );

    if let Some(sd) = &player.streaming_data {
        for f in sd.formats.iter().chain(sd.adaptive_formats.iter()) {
            println!(
                "itag {:>3} | {:<45} | {:>6} | url: {}",
                f.itag,
                f.mime_type,
                f.quality_label.as_deref().unwrap_or("-"),
                if f.url.is_some() { "yes" } else { "NO" },
            );
        }
    }
    Ok(Json(raw))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(|| async { "OK" }))
        .route("/api/debug/player", get(debug_player))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
