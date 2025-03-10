use crate::{AppError, AppState};
use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
};
use tera::Context;

pub async fn index(State(state): State<AppState>) -> Result<Response, AppError> {
    let mut ctx = Context::new();

    // Handle the Result from get_all_projects()
    let projects = crate::handlers::projects::get_all_projects()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    let career_timeline = crate::handlers::career_timeline::get_career_timeline()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    ctx.insert("projects", &projects);
    ctx.insert("career_timeline", &career_timeline);

    match state.tera.render("index.html", &ctx) {
        Ok(html) => Ok(Html(html).into_response()),
        Err(err) => Err(AppError::Template(err)),
    }
}
