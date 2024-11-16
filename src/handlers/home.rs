// src/handlers/home.rs
use crate::{AppError, AppState};
use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
};
use tera::Context;

pub async fn index(State(state): State<AppState>) -> Result<Response, AppError> {
    let mut ctx = Context::new();

    // Add projects data
    let projects = crate::handlers::projects::get_all_projects();
    ctx.insert("projects", &projects);

    match state.tera.render("index.html", &ctx) {
        Ok(html) => Ok(Html(html).into_response()), // Use Html type wrapper
        Err(err) => Err(AppError::Template(err)),
    }
}
