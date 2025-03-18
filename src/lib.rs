// src/lib.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tera::Tera;

pub mod handlers;
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
    pub tera: Arc<Tera>,
}

pub use crate::handlers::career_timeline::get_career_timeline;
pub use crate::handlers::home::index as home_index;
pub use crate::handlers::projects::{
    get_all_projects, get_related_projects, project_detail, Project,
};
