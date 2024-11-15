use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Template error: {0}")]
    Template(#[from] tera::Error),
    
    #[error("Project not found")]
    ProjectNotFound,
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Template(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::ProjectNotFound => (StatusCode::NOT_FOUND, "Project not found".to_string()),
            AppError::Internal(err) => (StatusCode::INTERNAL_SERVER_ERROR, err),
        };
        (status, message).into_response()
    }
}
