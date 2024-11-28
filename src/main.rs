// main.rs
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

mod handlers;
mod markdown;

#[derive(Debug)]
pub enum AppError {
    Template(tera::Error),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Template(err) => {
                eprintln!("Template error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Template error: {}", err),
                )
            }
            AppError::Internal(err) => {
                eprintln!("Internal error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, err)
            }
        };
        (status, error_message).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    tera: Arc<Tera>,
}

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

    // Add debug logging for Tera
    tera.full_reload().unwrap_or_else(|e| {
        eprintln!("Error reloading templates: {}", e);
        std::process::exit(1);
    });

    // Print all registered templates
    println!("Registered templates:");
    for template in tera.get_template_names() {
        println!("  - {}", template);
    }

    let state = AppState {
        tera: Arc::new(tera),
    };

    let app = Router::new()
        .route("/", get(handlers::home::index))
        .route("/projects", get(handlers::projects::index))
        .route("/projects/:id", get(handlers::projects::project_detail))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    println!("🔒 Initializing secure server on http://localhost:3000");
    println!("⚡ System status: ONLINE");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
