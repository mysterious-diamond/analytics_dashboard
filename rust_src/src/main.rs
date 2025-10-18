use axum::{routing::post, Router, Json};
use serde::{Serialize, Deserialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct AuthResult {
	known: bool,
}

#[tokio::main]
async fn main() {
    // Create router
    let app = Router::new().route("/verify", post(verify_user));

    // Bind TCP listener
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Rust API running at http://{}", addr);

    // Start the server
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

async fn verify_user(Json(data): Json<LoginData>) -> Json<AuthResult> {
	let valid = data.username == "aaron" && data.password == "123";
	println!("Got query, returning {}", valid);
	Json(AuthResult { known: valid })
}
