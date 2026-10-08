mod youtube;
mod routes;
use axum::{Json, Router, extract::Query, http::StatusCode, routing::get};
use serde::Deserialize;
use tower_http::services::ServeDir;
use youtube::{
    client::ClientProfile, innertube::fetch_player_with_fallback, models::PlayerResponse,
    video_id::parse_video_id,
};

#[derive(Deserialize)]
struct PlayerQuery {
    url: String,
}

async fn debug_player(
    Query(q): Query<PlayerQuery>,
) -> Result<Json<PlayerResponse>, (StatusCode, String)> {
    let id = parse_video_id(&q.url).map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    // A new client per request is wasteful;
    let http = reqwest::Client::new();

    let profiles = [ClientProfile::android_vr(), ClientProfile::ios()];
    let (player, used) = fetch_player_with_fallback(&http, &id, &profiles)
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("{e:#}")))?;

    println!("Client that worked: {}", used.name);
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
    Ok(Json(player))
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
