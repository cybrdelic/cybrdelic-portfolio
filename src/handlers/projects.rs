use crate::{AppError, AppState};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use tera::Context;

#[derive(Serialize, Clone)]
pub struct Command {
    text: String,
    description: String,
    icon_path: String,
}

#[derive(Serialize, Clone)]
pub struct FlowStep {
    title: String,
    description: String,
    command: Command,
}

#[derive(Serialize, Clone)]
pub struct UserFlow {
    title: String,
    content: String,
    steps: Vec<FlowStep>,
}

impl UserFlow {
    fn load_from_markdown(project_id: &str, flow_name: &str) -> Result<String, std::io::Error> {
        let mut path = PathBuf::from("content");
        path.push("projects");
        path.push(project_id);
        path.push("flows");
        path.push(format!("{}.md", flow_name));

        let content = fs::read_to_string(path)?;
        let processed_content = crate::markdown::preprocess_markdown(content);
        Ok(crate::markdown::parse_markdown(&processed_content))
    }
}

#[derive(Serialize, Clone)]
pub struct Project {
    id: String,
    title: String,
    subtitle: String,
    description: String,
    image_url: String,
    image_caption: String,
    icon_path: String,
    tech_stack: Vec<String>,
    links: Vec<ProjectLink>,
    features: Vec<Feature>,
    technical: TechnicalDetails,
    catchphrases: Vec<String>,
    user_flows: Vec<UserFlow>,
}

#[derive(Serialize, Clone)]
pub struct ProjectLink {
    title: String,
    url: String,
}

#[derive(Serialize, Clone)]
pub struct Feature {
    title: String,
    description: String,
    icon_path: String,
}

#[derive(Serialize, Clone)]
pub struct TechnicalDetails {
    architecture: String,
    implementation: String,
    challenges: String,
}

// updated Project::new to take autonomous flow configs as a Vec<(String, Vec<FlowStep>)>
impl Project {
    fn new(
        id: &str,
        title: &str,
        subtitle: &str,
        description: &str,
        image_url: &str,
        image_caption: &str,
        icon_path: &str,
        tech_stack: Vec<&str>,
        links: Vec<(&str, &str)>,
        features: Vec<(&str, &str, &str)>,
        technical: (&str, &str, &str),
        catchphrases: Vec<&str>,
        flow_configs: Vec<(String, Vec<FlowStep>)>,
    ) -> Result<Self, std::io::Error> {
        let user_flows = flow_configs
            .into_iter()
            .map(|(flow_name, steps)| {
                let content = UserFlow::load_from_markdown(id, &flow_name)?;
                Ok(UserFlow {
                    title: flow_name,
                    content,
                    steps,
                })
            })
            .collect::<Result<Vec<UserFlow>, std::io::Error>>()?;

        Ok(Self {
            id: id.to_string(),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            description: description.to_string(),
            image_url: image_url.to_string(),
            image_caption: image_caption.to_string(),
            icon_path: icon_path.to_string(),
            tech_stack: tech_stack.iter().map(|&s| s.to_string()).collect(),
            links: links
                .into_iter()
                .map(|(title, url)| ProjectLink {
                    title: title.to_string(),
                    url: url.to_string(),
                })
                .collect(),
            features: features
                .into_iter()
                .map(|(title, description, icon_path)| Feature {
                    title: title.to_string(),
                    description: description.to_string(),
                    icon_path: icon_path.to_string(),
                })
                .collect(),
            technical: TechnicalDetails {
                architecture: technical.0.to_string(),
                implementation: technical.1.to_string(),
                challenges: technical.2.to_string(),
            },
            catchphrases: catchphrases.iter().map(|&s| s.to_string()).collect(),
            user_flows,
        })
    }
}

// new function to autonomously scan the flows directory and build a vector of (flow_name, empty steps)
pub fn scan_flow_configs(project_id: &str) -> Result<Vec<(String, Vec<FlowStep>)>, std::io::Error> {
    let mut configs = Vec::new();
    let dir_path = PathBuf::from("content")
        .join("projects")
        .join(project_id)
        .join("flows");
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "md" {
                    if let Some(stem) = path.file_stem() {
                        let flow_name = stem.to_string_lossy().into_owned();
                        configs.push((flow_name, Vec::new()));
                    }
                }
            }
        }
    }
    Ok(configs)
}

pub async fn project_detail(
    State(state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Response, AppError> {
    let mut ctx = Context::new();

    let project = get_project_by_id(&project_id)
        .ok_or_else(|| AppError::Internal(format!("Project not found: {}", project_id)))?;

    let related_projects = get_related_projects(&project_id);

    ctx.insert("project", &project);
    ctx.insert("related_projects", &related_projects);

    match state.tera.render("project_detail.html", &ctx) {
        Ok(html) => Ok(Html(html).into_response()),
        Err(err) => Err(AppError::Template(err)),
    }
}

pub async fn index(State(state): State<AppState>) -> Result<Response, AppError> {
    let mut ctx = Context::new();

    match get_all_projects() {
        Ok(projects) => {
            ctx.insert("projects", &projects);
            match state.tera.render("sections/projects.html", &ctx) {
                Ok(html) => Ok(Html(html).into_response()),
                Err(err) => Err(AppError::Template(err)),
            }
        }
        Err(e) => Err(AppError::Internal(e.to_string())),
    }
}

pub fn get_all_projects() -> Result<Vec<Project>, std::io::Error> {
    let sagacity_flow_configs = scan_flow_configs("sagacity")?;
    let commitaura_flow_configs = scan_flow_configs("commitaura")?;
    let cybrdelic_flow_configs = scan_flow_configs("cybrdelic-portfolio")?;
    Ok(vec![
        Project::new(
            "sagacity",
            "sagacity",
            "intelligent software copilot",
            "intelligent codebase exploration tool using natural language interaction. features novel summarizational indexing system and persistent context memory.",
            "/static/images/sagacity.jpg",
            "intelligent codebase exploration tool powered by claude api",
            "m20 21v-2a4 4 0 0 0-4-4h8a4 4 0 0 0-4 4v2 m12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8z",
            vec!["rust", "claude api", "nlp", "git"],
            vec![
                ("crates.io", "https://crates.io/crates/sagacity"),
                ("github", "https://github.com/cybrdelic/sagacity"),
            ],
            vec![
                (
                    "natural language search",
                    "query your codebase using natural language and get contextually relevant results",
                    "m21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                ),
                (
                    "context memory",
                    "maintains conversation context for more intelligent interactions",
                    "m12 6.253v13m0-13c10.832 5.477 9.246 5 7.5 5s4.168 5.477 3 6.253v13c4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253"
                ),
                (
                    "ai-powered indexing",
                    "smart indexing system that understands code semantics",
                    "m9 12h6m-6 4h6m2 5h7a2 2 0 01-2-2v5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707v19a2 2 0 01-2 2z"
                ),
            ],
            (
                "built with a microservices architecture using rust for performance and reliability. integrates with claude api for intelligent processing.",
                "uses advanced nlp techniques and custom indexing algorithms. implements persistent storage with sqlite.",
                "optimizing response times while maintaining context. balancing memory usage with search performance."
            ),
            vec!["intelligent search", "code understanding", "developer focus", "efficiency"],
            sagacity_flow_configs,
        )?,
        Project::new(
            "commitaura",
            "commitaura",
            "ai-powered commit message generator",
            "autonomously generate commit messages for your staged commits, using diff analysis as contextualization.",
            "/static/images/commitaura.gif",
            "ai-powered commit message generation in action",
            "m6 3v12 m18 6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z m6 18a3 3 0 1 0 0-6 3 3 0 0 0 0 6z m18 9a9 9 0 0 1-9 9",
            vec!["rust", "claude api", "git"],
            vec![
                ("view documentation", "https://docs.commitaura.dev"),
                ("view source", "https://github.com/cybrdelic/commitaura"),
            ],
            vec![
                (
                    "claude integration",
                    "analyzes git diffs using the claude api to understand code changes contextually",
                    "m8 7v8a2 2 0 002 2h6m8 7v5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707v15a2 2 0 01-2 2h-2"
                ),
                (
                    "smart commit messages",
                    "generates meaningful commit messages using context-aware analysis",
                    "m8 12h.01m12 12h.01m16 12h.01m21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949l3 20l1.395-3.72c3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                ),
            ],
            (
                "event-driven architecture with git hook integration. uses rust async runtime for performance.",
                "implements custom diff parsing and claude prompt engineering. features caching and rate limiting.",
                "handling complex git histories and merge commits. ensuring consistent message quality."
            ),
            vec!["git integration", "ai-powered", "developer workflow", "productivity"],
            commitaura_flow_configs,
        )?,
        Project::new(
            "cybrdelic-portfolio",
            "cybrdelic portfolio",
            "interactive developer portfolio",
            "a full-stack portfolio and interactive web app built with rust, featuring dynamic routing, interactive ui elements, and markdown-driven documentation. integrates axum, tera, and modern javascript to deliver a cutting-edge, terminal-inspired experience.",
            "/static/images/cybrdelic-portfolio.jpg",
            "an interactive showcase of cutting-edge developer projects with terminal vibes",
            "m10 10h4v4h-4z",
            vec!["rust", "axum", "tera", "javascript", "css"],
            vec![
                ("github", "https://github.com/cybrdelic/cybrdelic-portfolio"),
                ("demo", "https://cybrdelic-portfolio.example.com"),
            ],
            vec![
                (
                    "dynamic routing",
                    "seamless navigation with axum routing and modular content rendering",
                    "m5 5h14v14h-14z"
                ),
                (
                    "interactive ui",
                    "rich animations, transitions, and terminal-inspired design for a modern feel",
                    "m2 2h20v20h-20z"
                ),
                (
                    "markdown driven",
                    "dynamic parsing and rendering of markdown for user flows and documentation",
                    "m4 4h16v16h-16z"
                ),
            ],
            (
                "rust backend with axum",
                "tera templating for dynamic content",
                "robust error handling and markdown processing"
            ),
            vec!["cutting-edge", "interactive", "modern", "modular"],
            cybrdelic_flow_configs,
        )?,
    ])
}

fn get_project_by_id(id: &str) -> Option<Project> {
    get_all_projects().ok()?.into_iter().find(|p| p.id == id)
}

pub fn get_related_projects(current_id: &str) -> Vec<Project> {
    get_all_projects()
        .unwrap_or_default()
        .into_iter()
        .filter(|p| p.id != current_id)
        .take(3)
        .collect()
}
