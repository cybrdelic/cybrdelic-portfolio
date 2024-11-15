use axum::{extract::State, response::Html};
use crate::{AppState, AppError};

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let mut context = tera::Context::new();
    context.insert("page_title", "Cybrdelic :: Home");
    context.insert("last_update", "2024-03-14");
    
    state.tera
        .render("index.html", &context)
        .map(Html)
        .map_err(AppError::Template)
}
