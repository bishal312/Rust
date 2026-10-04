use axum::{Router, routing::get};
use tower_http::services::ServeDir;
mod youtube;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/health", get(|| async { "OK" }))
        .fallback_service(ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
