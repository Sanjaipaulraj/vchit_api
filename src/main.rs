use axum::{
    Router,
    routing::{get, post},
};
mod handlers;
mod models;

use handlers::{collections::get_collections, members::get_members,groups::get_groups};
use tower_http::cors::{CorsLayer,Any};

#[tokio::main]
async fn main() {

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", post(ping))
        .route("/collectionlist", get(get_collections))
        .route("/members", get(get_members))
        .route("/groups", get(get_groups))
        .layer(cors);

    let ip_port = format!("0.0.0.0:5000");
    println!("Server start at {}", ip_port);
    let listener = tokio::net::TcpListener::bind(ip_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping(msg: String) -> String {
    if msg == "ping" {
        return format!("Pong");
    }
    return format!("Invalid Message");
}
