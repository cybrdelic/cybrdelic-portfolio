// src/handlers/static_files.rs
use axum::response::Html;

pub async fn serve_static_file(path: &str) -> Html<&'static str> {
    match path {
        "/index.html" => Html(include_str!("../../static/index.html")),
        "/project-template.html" => Html(include_str!("../../static/project-template.html")),
        _ => Html("<h1>404 Not Found</h1>"),
    }
}
