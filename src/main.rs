// src/main.rs
use axum::{
    extract::State,
    routing::{get, post},
    Router,
};
use cybrdelic_portfolio::{home_index, project_detail, AppError, AppState};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Template parsing error: {}", e);
            std::process::exit(1);
        }
    };

    tera.full_reload().unwrap_or_else(|e| {
        eprintln!("Error reloading templates: {}", e);
        std::process::exit(1);
    });

    println!("Registered templates:");
    for template in tera.get_template_names() {
        println!("  - {}", template);
    }

    let state = AppState {
        tera: Arc::new(tera),
    };

    let app = Router::new()
        .route("/", get(home_index))
        .route(
            "/projects",
            get(cybrdelic_portfolio::handlers::projects::index),
        )
        .route("/projects/:id", get(project_detail))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    println!("🔒 Initializing secure server on http://localhost:3000");
    println!("⚡ System status: ONLINE");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
