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

// Cache for markdown content to avoid repeated file reads and parsing
use std::collections::HashMap;
static MARKDOWN_CACHE: Lazy<Mutex<HashMap<String, String>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

impl UserFlow {
    fn load_from_markdown(project_id: &str, flow_name: &str) -> Result<String, std::io::Error> {
        let cache_key = format!("{}-{}", project_id, flow_name);

        // Check cache first
        {
            let cache = MARKDOWN_CACHE.lock().unwrap();
            if let Some(content) = cache.get(&cache_key) {
                return Ok(content.clone());
            }
        }

        // If not in cache, load from file
        let mut path = PathBuf::from("content");
        path.push("projects");
        path.push(project_id);
        path.push("flows");
        path.push(format!("{}.md", flow_name));

        // Add path check to avoid redundant filesystem operations if file doesn't exist
        if !path.exists() {
            let default_content = String::from("<p>Documentation coming soon...</p>");
            // Cache the default content too
            let mut cache = MARKDOWN_CACHE.lock().unwrap();
            cache.insert(cache_key, default_content.clone());
            return Ok(default_content);
        }

        let content = fs::read_to_string(path)?;
        let processed_content = crate::markdown::preprocess_markdown(content);
        let html = crate::markdown::parse_markdown(&processed_content);

        // Cache the result
        let mut cache = MARKDOWN_CACHE.lock().unwrap();
        cache.insert(cache_key, html.clone());

        Ok(html)
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

    // Optimization: For fast initial rendering, we load basic project info first
    let project = get_project_by_id(&project_id)
        .ok_or_else(|| AppError::Internal(format!("Project not found: {}", project_id)))?;

    // Get related projects (these are also fast to load, no markdown)
    let related_projects = get_related_projects(&project_id);

    ctx.insert("project", &project);
    ctx.insert("related_projects", &related_projects);

    // Add preloading hint for browsers
    ctx.insert("preload_hint", &true);

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

// Add a static memoization for projects to avoid scanning on every request
use once_cell::sync::Lazy;
use std::sync::Mutex;

static PROJECTS_CACHE: Lazy<Mutex<Option<Vec<Project>>>> = Lazy::new(|| Mutex::new(None));

pub fn get_all_projects() -> Result<Vec<Project>, std::io::Error> {
    // Check if we have a cached result
    let mut cache = PROJECTS_CACHE.lock().unwrap();
    if let Some(projects) = cache.clone() {
        return Ok(projects);
    }

    // If not cached, build the projects list
    let myriad_flow_configs = scan_flow_configs("myriad")?;
    let commitaura_flow_configs = scan_flow_configs("commitaura")?;
    let cybrdelic_flow_configs = scan_flow_configs("cybrdelic-portfolio")?;
    let jjugg_flow_configs = scan_flow_configs("jjugg")?;
    let resumatyk_flow_configs = scan_flow_configs("resumatyk")?;
    let browsealizer_flow_configs = scan_flow_configs("browsealizer")?;
    let projects_result = vec![
        Project::new(
            "jjugg",
            "JJugg",
            "Autonomous Job Application Tracker",
            "A local web application for autonomously tracking job applications that transforms a traditionally manual process into a data-driven workflow with automated tracking (via userscripts and email scanning), rich analytics, and interactive dashboards.Think of it like JARVIS for job hunters.",
            "/static/images/jjugg.jpg",
            "Job application tracking simplified with automation and analytics",
            "m5 5h14a2 2 0 012 2v10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2z m13 3l-5 5-5-5",
            vec!["JavaScript", "HTML", "CSS", "LocalStorage", "User Scripts"],
            vec![
                ("GitHub", "https://github.com/cybrdelic/jjugg"),
                ("Demo", "https://jjugg.example.com"),
            ],
            vec![
                (
                    "Automated Tracking",
                    "Uses userscripts with localStorage and email scanning to automatically update application statuses, so you don't need ot manually enter any data - just apply and your data will be autonomously tracked.",
                    "m9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                ),
                (
                    "Rich Dashboard",
                    "Comprehensive analytics, interactive Kanban boards, tables, and detailed profiles with project information",
                    "m9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                ),
                (
                    "AI-Driven Insights",
                    "Future integration of AI-generated insights and personalized job recommendations",
                    "m8 9l3 3-3 3m5 0h3M5 20h14a2 2 0 002-2V6a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"
                ),
            ],
            (
                "Client-side architecture with automated data collection and persistence using localStorage and browser userscripts.",
                "Custom data visualization, integration with email scanning APIs, and interactive dashboards.",
                "Handling inconsistent application status formats across different platforms and maintaining data integrity."
            ),
            vec!["automated", "data-driven", "insightful", "time-saving"],
            jjugg_flow_configs,
        )?,

        Project::new(
            "myriad",
            "Myriad.ai",
            "Intelligent Software Copilot",
            "An AI-powered code exploration tool that helps developers navigate and understand complex codebases, and produce project planning/documentation through contextual code analysis and natural language interaction.",
            "/static/images/myriad.jpg",
            "Intelligent codebase exploration tool powered by Claude API",
            "m20 21v-2a4 4 0 0 0-4-4h8a4 4 0 0 0-4 4v2 m12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8z",
            vec!["Rust", "Claude API", "NLP", "Git"],
            vec![
                ("Crates.io", "https://crates.io/crates/myriad"),
                ("GitHub", "https://github.com/cybrdelic/myriad"),
            ],
            vec![
                (
                    "Natural Language Search",
                    "Query your codebase using natural language and get contextually relevant results",
                    "m21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                ),
                (
                    "Context Memory",
                    "Maintains conversation context for more intelligent interactions",
                    "m12 6.253v13m0-13c10.832 5.477 9.246 5 7.5 5s4.168 5.477 3 6.253v13c4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253"
                ),
                (
                    "AI-Powered Contextual Indexing",
                    "Smart indexing system that understands user prompts and where the related code is",
                    "m9 12h6m-6 4h6m2 5h7a2 2 0 01-2-2v5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707v19a2 2 0 01-2 2z"
                ),
            ],
            (
                "Built as a TUI using Rust for performance. Integrates with Claude API for intelligent processing.",
                "Uses custom indexing and contextual ranking algorithms. Implements persistent storage with SQLite.",
                "Optimizing response times while maintaining context. Balancing memory usage with search performance.",

            ),
            vec!["intelligent search", "code understanding", "developer focus", "efficiency"],
            myriad_flow_configs,
        )?,

        Project::new(
            "commitaura",
            "CommitAura",
            "AI-Powered Commit Message Generator",
            "A Rust-based CLI tool that analyzes Git commit patterns, streamlines version control workflows, and autonomously generates meaningful commit messages based on code changes.",
            "/static/images/commitaura.gif",
            "AI-powered commit message generation in action",
            "m6 3v12 m18 6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z m6 18a3 3 0 1 0 0-6 3 3 0 0 0 0 6z m18 9a9 9 0 0 1-9 9",
            vec!["Rust", "Claude API", "Git"],
            vec![
                ("Documentation", "https://docs.commitaura.dev"),
                ("GitHub", "https://github.com/cybrdelic/commitaura"),
            ],
            vec![
                (
                    "Claude Integration",
                    "Analyzes git diffs using the Claude API to understand code changes contextually",
                    "m8 7v8a2 2 0 002 2h6m8 7v5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707v15a2 2 0 01-2 2h-2"
                ),
                (
                    "Smart Commit Messages",
                    "Generates meaningful commit messages using context-aware analysis",
                    "m8 12h.01m12 12h.01m16 12h.01m21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949l3 20l1.395-3.72c3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                ),
                (
                    "Semantic Analysis",
                    "Detects and suggests improvements for commit messages to ensure clarity and consistency",
                    "m11 4a2 2 0 114 0v1a1 1 0 001 1h3a1 1 0 011 1v3a1 1 0 01-1 1h-1a2 2 0 100 4h1a1 1 0 011 1v3a1 1 0 01-1 1h-3a1 1 0 01-1-1v-1a2 2 0 10-4 0v1a1 1 0 01-1 1H7a1 1 0 01-1-1v-3a1 1 0 00-1-1H4a2 2 0 110-4h1a1 1 0 001-1V7a1 1 0 011-1h3a1 1 0 001-1V4z"
                ),
            ],
            (
                "Event-driven architecture with Git hook integration. Uses Rust async runtime for performance.",
                "Implements custom diff parsing and Claude prompt engineering. Features caching and rate limiting.",
                "Handling complex Git histories and merge commits. Ensuring consistent message quality."
            ),
            vec!["git integration", "AI-powered", "developer workflow", "productivity"],
            commitaura_flow_configs,
        )?,

        Project::new(
            "resumatyk",
            "Resumatyk",
            "AI Resume Management TUI",
            "A resume management system (written in Bash using fzf) that allows the creation, editing, compilation, and viewing of resumes. Also enables resume style variant generation using LaTeX code generation, OCR, and fully autonomous, multimodal, LLM-based, recursive QA flows.",
            "/static/images/resumatyk.jpg",
            "Generate professional LaTeX resumes tailored to specific job requirements",
            "m4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4",
            vec!["Python", "LaTeX", "OCR", "AI"],
            vec![
                ("GitHub", "https://github.com/cybrdelic/resumatyk"),
                ("Demo", "https://resumatyk.example.com"),
            ],
            vec![
                (
                    "Dynamic Resume Generation",
                    "Creates AI-styled resume variants on the fly to match specific job requirements",
                    "m8 7v8a2 2 0 002 2h6m8 7v5a2 2 0 012-2h.93l.57-4H2.5l.57 4H4a2 2 0 012 2v5a2 2 0 01-2 2H2a2 2 0 01-2-2v-5a2 2 0 012-2h2.93l.57-4H20.5l.57 4H22a2 2 0 012 2v5a2 2 0 01-2 2h-2a2 2 0 01-2-2v-5a2 2 0 012-2h1.93l.57-4"
                ),
                (
                    "LaTeX Styling",
                    "Ensures professional formatting and high-quality typesetting for polished resumes",
                    "m15 5v2m0 4v2m0 4v2M5 5a2 2 0 00-2 2v3a2 2 0 110 4v3a2 2 0 002 2h14a2 2 0 002-2v-3a2 2 0 110-4V7a2 2 0 00-2-2H5z"
                ),
                (
                    "OCR Integration",
                    "Integrates OCR tools and structured content extraction to adapt resumes dynamically",
                    "m7 21h10a2 2 0 002-2V9.414a1 1 0 00-.293-.707l-5.414-5.414A1 1 0 0012.586 3H7a2 2 0 00-2 2v14a2 2 0 002 2z"
                ),
            ],
            (
                "Multi-component architecture with LaTeX template engine, OCR processing, and AI content generation.",
                "Python-based document processing, OCR libraries for content extraction, and AI integration for content adaptation.",
                "Maintaining consistent formatting across different resume types and ensuring compatibility with various job application systems."
            ),
            vec!["automated", "professional", "dynamic", "tailored"],
            resumatyk_flow_configs,
        )?,

        Project::new(
            "cybrdelic-portfolio",
            "Cybrdelic Portfolio",
            "Interactive Developer Portfolio",
            "A full-stack portfolio and interactive web app built with Rust, featuring dynamic routing, interactive UI elements, and markdown-driven documentation. Integrates Axum, Tera, and modern JavaScript to deliver a cutting-edge, cyberpunk-inspired experience.",
            "/static/images/cybrdelic-portfolio.jpg",
            "An interactive showcase of cutting-edge developer projects with terminal vibes",
            "m10 10h4v4h-4z",
            vec!["Rust", "Axum", "Tera", "JavaScript", "CSS"],
            vec![
                ("GitHub", "https://github.com/cybrdelic/cybrdelic-portfolio"),
                ("Demo", "https://cybrdelic-portfolio.example.com"),
            ],
            vec![
                (
                    "Dynamic Routing",
                    "Seamless navigation with Axum routing and modular content rendering",
                    "m5 5h14v14h-14z"
                ),
                (
                    "Interactive UI",
                    "Rich animations, transitions, and terminal-inspired design for a modern feel",
                    "m2 2h20v20h-20z"
                ),
                (
                    "Markdown Driven",
                    "Dynamic parsing and rendering of markdown for user flows and documentation",
                    "m4 4h16v16h-16z"
                ),
            ],
            (
                "Rust backend with Axum web framework for high-performance, reliable server operations.",
                "Tera templating for dynamic content rendering, modern CSS architecture, and vanilla JavaScript for interactivity.",
                "Balancing design aesthetics with performance, handling markdown processing for documentation, and ensuring responsive design."
            ),
            vec!["cutting-edge", "interactive", "modern", "modular"],
            cybrdelic_flow_configs,
        )?,

        Project::new(
            "browsealizer",
            "Browsealizer",
            "GitHub Project Explorer",
            "A tool designed to enhance the GitHub browsing experience by offering an unlimited, continuously updated feed of projects with mobile optimization and intuitive filtering.",
            "/static/images/browsealizer.jpg",
            "Discover GitHub projects with an endless, mobile-friendly feed",
            "m21 12a9 9 0 01-9 9m9-9a9 9 0 00-9-9m9 9H3m9 9a9 9 0 01-9-9m9 9c1.657 0 3-4.03 3-9s-1.343-9-3-9m0 18c-1.657 0-3-4.03-3-9s1.343-9 3-9m-9 9a9 9 0 019-9",
            vec!["React", "TypeScript", "GitHub API", "CSS"],
            vec![
                ("GitHub", "https://github.com/cybrdelic/browsealizer"),
                ("Live Demo", "https://browsealizer.example.com"),
            ],
            vec![
                (
                    "Unlimited Feed",
                    "Provides a seamless, endless stream of GitHub repositories for exploration",
                    "m4 8v4m0 4h.01M4 20h4m0-4h12a4 4 0 00-4-4H8m0-4h12a4 4 0 00-4-4H8"
                ),
                (
                    "Mobile Optimization",
                    "Ensures that the browsing experience is smooth on both desktop and mobile devices",
                    "m12 18h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"
                ),
                (
                    "User-Centric Interface",
                    "Intuitive filtering and project discovery features that keep users engaged",
                    "m3 4h13M3 8h9m-9 4h9m5-4v12m0 0l-4-4m4 4l4-4"
                ),
            ],
            (
                "React-based frontend with TypeScript for type safety and robust component architecture.",
                "GitHub API integration for project data with client-side filtering and continuous loading capabilities.",
                "Efficiently handling API rate limits, ensuring responsive performance on mobile devices, and delivering a smooth infinite scrolling experience."
            ),
            vec!["discovery", "mobile-friendly", "continuous", "user-friendly"],
            browsealizer_flow_configs,
        )?,
    ];

    // Cache the result
    *cache = Some(projects_result.clone());

    Ok(projects_result)
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
