// src/routes/mod.rs
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::ServeDir;

use crate::handlers::{contact, projects, static_files};

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(projects::serve_index))
        .route("/api/projects", get(projects::get_projects))
        .route("/api/projects/:id", get(projects::get_project_details))
        .route("/project/:id", get(projects::serve_project_page))
        .route("/api/contact", post(contact::handle_contact))
        .nest_service("/static", ServeDir::new("static"))
}
