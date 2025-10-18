use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Stats {
    users: u32,
    messages: u32,
}

async fn get_stats() -> Json<Stats> {
    let stats = Stats {
        users: 42,
        messages: 128,
    };

    println!("Got message, returning stats");
    Json(stats)
}

#[tokio::main]
async fn main() {
    // Create router
    let app = Router::new().route("/stats", get(get_stats));

    // Bind TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Rust API running at http://{}", addr);

    // Start the server
    axum::serve(listener, app)
        .await
        .unwrap();
}
