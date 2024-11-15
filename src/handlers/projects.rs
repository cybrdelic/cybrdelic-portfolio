use axum::{
    extract::{State, Path},
    response::{Html, Json},
};
use crate::{AppState, AppError};
use crate::config::get_project_by_slug;

pub async fn show(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Html<String>, AppError> {
    let project = get_project_by_slug(&slug)
        .ok_or(AppError::ProjectNotFound)?;

    let mut context = tera::Context::new();
    context.insert("content", &state.content);
    context.insert("project", &project);
    context.insert("current_page", "project");
    
    state.tera
        .render("pages/project.html", &context)
        .map(Html)
        .map_err(AppError::Template)
}

pub async fn show_json(
    Path(slug): Path<String>,
) -> Result<Json<config::ProjectDetail>, AppError> {
    let project = get_project_by_slug(&slug)
        .ok_or(AppError::ProjectNotFound)?;
    
    Ok(Json(project))
}
