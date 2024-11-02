use axum::{
    routing::{get, post},
    Router,
    response::Html,
    http::StatusCode,
    Json,
};
use tower_http::services::ServeDir;
use serde::Serialize;

#[derive(Serialize)]
struct Project {
    title: String,
    description: String,
    technologies: Vec<String>,
    github_url: String,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(serve_index))
        .route("/api/projects", get(get_projects))
        .route("/api/contact", post(handle_contact))
        .nest_service("/static", ServeDir::new("static"));

    println!("Server running on http://localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../static/index.html"))
}

async fn get_projects() -> Json<Vec<Project>> {
    let projects = vec![
        Project {
            title: "Commitaura".to_string(),
            description: "AI-powered commit message generator using OpenAI's GPT API".to_string(),
            technologies: vec!["Rust".to_string(), "OpenAI".to_string(), "CLI".to_string()],
            github_url: "https://github.com/cybrdelic/commitaura".to_string(),
        },
        Project {
            title: "Sagacity".to_string(),
            description: "Codebase exploration tool powered by Claude AI".to_string(),
            technologies: vec!["Rust".to_string(), "Claude AI".to_string(), "CLI".to_string()],
            github_url: "https://github.com/cybrdelic/sagacity".to_string(),
        },
    ];
    Json(projects)
}

async fn handle_contact() -> StatusCode {
    StatusCode::OK
}
