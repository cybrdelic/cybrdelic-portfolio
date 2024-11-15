use axum::extract::State;
use axum::response::{IntoResponse, Response};

use crate::{AppError, AppState};

pub async fn index(State(state): State<AppState>) -> Result<Response, AppError> {
    let mut ctx = tera::Context::new();

    match state.tera.render("projects.html", &ctx) {
        Ok(html) => Ok(html.into_response()),
        Err(err) => Err(AppError::Template(err)),
    }
}
