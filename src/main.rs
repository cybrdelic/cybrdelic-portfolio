// src/main.rs
use crate::handlers::{contact, projects, static_files};
use crate::setup::app_config;

mod handlers;
mod models;
mod routes;
mod setup;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Setup application router
    let app = app_config::create_router();

    // Start server
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("Server running on http://localhost:3000");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app.into_make_service(),
    )
    .await
    .unwrap();
}
