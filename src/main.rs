use axum::{
    routing::{get, post},
    Router,
    extract::State,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use std::sync::Arc;
use tera::Tera;
use tower_http::services::ServeDir;

mod handlers;

#[derive(Debug)]
pub enum AppError {
    Template(tera::Error),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Template(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };
        (status, message).into_response()
    }
}

#[derive(Clone)]
pub struct AppState {
    tera: Arc<Tera>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let tera = match Tera::new("templates/**/*.html") {
        Ok(t) => Arc::new(t),
        Err(e) => {
            eprintln!("Error parsing templates: {}", e);
            std::process::exit(1);
        }
    };

    let state = AppState { tera };

    let app = Router::new()
        .route("/", get(handlers::home::index))
        .route("/contact", post(handlers::contact::submit))
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    println!("🔒 Initializing secure server on http://localhost:3000");
    println!("⚡ System status: ONLINE");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
