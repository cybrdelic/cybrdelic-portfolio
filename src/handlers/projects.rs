use crate::{AppError, AppState};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;
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
    description: String,
    steps: Vec<FlowStep>,
}

#[derive(Serialize, Clone)]
pub struct Project {
    id: String,
    title: String,
    subtitle: String,
    description: String,
    image_url: String,
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

#[derive(Serialize, Clone)]
pub struct UserFlowCommand {
    text: String,
    description: String,
    icon_path: String,
}

impl Project {
    fn new(
        id: &str,
        title: &str,
        subtitle: &str,
        description: &str,
        image_url: &str,
        icon_path: &str,
        tech_stack: Vec<&str>,
        links: Vec<(&str, &str)>,
        features: Vec<(&str, &str, &str)>,
        technical: (&str, &str, &str),
        catchphrases: Vec<&str>,
        user_flows: Vec<UserFlow>,
    ) -> Self {
        Self {
            id: id.to_string(),
            title: title.to_string(),
            subtitle: subtitle.to_string(),
            description: description.to_string(),
            image_url: image_url.to_string(),
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
        }
    }
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
    ctx.insert("projects", &get_all_projects());

    match state.tera.render("sections/projects.html", &ctx) {
        Ok(html) => Ok(Html(html).into_response()),
        Err(err) => Err(AppError::Template(err)),
    }
}

pub fn get_all_projects() -> Vec<Project> {
    vec![
        Project::new(
            "sagacity",
            "Sagacity",
            "Rust / Claude API",
            "Intelligent codebase exploration tool using natural language interaction. Features novel summarizational indexing system and persistent context memory.",
            "/images/sagacity-demo.gif",
            "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2 M12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8z",
            vec!["Rust", "Claude API", "NLP", "Git"],
            vec![
                ("crates.io", "https://crates.io/crates/sagacity"),
                ("github", "https://github.com/yourusername/sagacity"),
            ],
            vec![
                (
                    "Natural Language Search",
                    "Query your codebase using natural language and get contextually relevant results",
                    "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z",
                ),
                (
                    "Context Memory",
                    "Maintains conversation context for more intelligent interactions",
                    "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253",
                ),
                (
                    "AI-Powered Indexing",
                    "Smart indexing system that understands code semantics",
                    "M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
                ),
            ],
            (
                "Built with a microservices architecture using Rust for performance and reliability. Integrates with Claude API for intelligent processing.",
                "Uses advanced NLP techniques and custom indexing algorithms. Implements persistent storage with SQLite.",
                "Optimizing response times while maintaining context. Balancing memory usage with search performance.",
            ),
            vec!["Intelligent Search", "Code Understanding", "Developer Focus", "Efficiency"],
            vec![
                UserFlow {
                    title: "Installation".to_string(),
                    description: "Getting started with Sagacity involves a few simple steps. Follow this guide to set up the tool in your development environment.".to_string(),
                    steps: vec![
                        FlowStep {
                            title: "Install Rust".to_string(),
                            description: "Install the Rust programming language and Cargo package manager".to_string(),
                            command: Command {
                                text: "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh".to_string(),
                                description: "Install Rust toolchain".to_string(),
                                icon_path: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Install Sagacity".to_string(),
                            description: "Install Sagacity using Cargo, Rust's package manager".to_string(),
                            command: Command {
                                text: "cargo install sagacity".to_string(),
                                description: "Install Sagacity via Cargo".to_string(),
                                icon_path: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Initialize Project".to_string(),
                            description: "Set up Sagacity in your development environment".to_string(),
                            command: Command {
                                text: "sagacity init".to_string(),
                                description: "Initialize Sagacity in your project".to_string(),
                                icon_path: "M12 6v6m0 0v6m0-6h6m-6 0H6".to_string(),
                            },
                        },
                    ],
                },
                UserFlow {
                    title: "Basic Usage".to_string(),
                    description: "Learn how to use Sagacity's core features for exploring and understanding your codebase.".to_string(),
                    steps: vec![
                        FlowStep {
                            title: "Search Codebase".to_string(),
                            description: "Search through your codebase using natural language queries".to_string(),
                            command: Command {
                                text: "sagacity search \"find error handling patterns\"".to_string(),
                                description: "Search codebase for patterns".to_string(),
                                icon_path: "M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Analyze Code".to_string(),
                            description: "Get detailed analysis of specific functionality".to_string(),
                            command: Command {
                                text: "sagacity analyze \"explain auth flow\"".to_string(),
                                description: "Analyze specific functionality".to_string(),
                                icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Use Context".to_string(),
                            description: "Leverage contextual understanding for better insights".to_string(),
                            command: Command {
                                text: "sagacity context \"how does this relate to user model?\"".to_string(),
                                description: "Get contextual information".to_string(),
                                icon_path: "M13 10V3L4 14h7v7l9-11h-7z".to_string(),
                            },
                        },
                    ],
                },
            ],
        ),Project::new(
            "commitaura",
            "Commitaura",
            "Rust / GPT-4",
            "Autonomous commit message generator using Git diff analysis and contextual understanding.",
            "/static/images/commitaura.jpg",
            "M6 3v12 M18 6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M6 18a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M18 9a9 9 0 0 1-9 9",
            vec!["Rust", "GPT-4", "Git"],
            vec![
                ("crates.io", "https://crates.io/crates/commitaura"),
                ("github", "https://github.com/yourusername/commitaura"),
            ],
            vec![
                (
                    "Smart Diff Analysis",
                    "Analyzes git diffs to understand code changes contextually",
                    "M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2M8 7H6a2 2 0 00-2 2v10a2 2 0 002 2h8a2 2 0 002-2v-2",
                ),
                (
                    "AI Message Generation",
                    "Generates meaningful commit messages using GPT-4",
                    "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z",
                ),
                (
                    "Change Detection",
                    "Intelligent system for detecting significant code changes",
                    "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2",
                ),
            ],
            (
                "Event-driven architecture with Git hook integration. Uses Rust async runtime for performance.",
                "Implements custom diff parsing and GPT-4 prompt engineering. Features caching and rate limiting.",
                "Handling complex Git histories and merge commits. Ensuring consistent message quality across different types of changes.",
            ),
            vec!["Automated Workflow", "Smart Commits", "Git Integration", "AI-Powered"],
            vec![
                UserFlow {
                    title: "Setup & Configuration".to_string(),
                    description: "Set up Commitaura in your repository and configure it for your workflow.".to_string(),
                    steps: vec![
                        FlowStep {
                            title: "Install Commitaura".to_string(),
                            description: "Install the tool globally using Cargo package manager".to_string(),
                            command: Command {
                                text: "cargo install commitaura".to_string(),
                                description: "Install Commitaura globally".to_string(),
                                icon_path: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Initialize Repository".to_string(),
                            description: "Set up Commitaura configuration in your Git repository".to_string(),
                            command: Command {
                                text: "commitaura init".to_string(),
                                description: "Initialize in repository".to_string(),
                                icon_path: "M12 6v6m0 0v6m0-6h6m-6 0H6".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Install Git Hooks".to_string(),
                            description: "Set up Git hooks for automated commit message generation".to_string(),
                            command: Command {
                                text: "commitaura hooks install".to_string(),
                                description: "Install Git hooks".to_string(),
                                icon_path: "M13 10V3L4 14h7v7l9-11h-7z".to_string(),
                            },
                        },
                    ],
                },
                UserFlow {
                    title: "Daily Workflow".to_string(),
                    description: "Learn how to use Commitaura in your daily development workflow for generating intelligent commit messages.".to_string(),
                    steps: vec![
                        FlowStep {
                            title: "Stage Changes".to_string(),
                            description: "Stage your code changes for commit".to_string(),
                            command: Command {
                                text: "git add .".to_string(),
                                description: "Stage your changes".to_string(),
                                icon_path: "M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Generate Message".to_string(),
                            description: "Generate an AI-powered commit message based on your changes".to_string(),
                            command: Command {
                                text: "commitaura suggest".to_string(),
                                description: "Generate commit message".to_string(),
                                icon_path: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Commit Changes".to_string(),
                            description: "Commit your changes with the generated message".to_string(),
                            command: Command {
                                text: "commitaura commit".to_string(),
                                description: "Commit with generated message".to_string(),
                                icon_path: "M5 13l4 4L19 7".to_string(),
                            },
                        },
                    ],
                },
                UserFlow {
                    title: "Customization".to_string(),
                    description: "Customize Commitaura's behavior to match your team's commit style and requirements.".to_string(),
                    steps: vec![
                        FlowStep {
                            title: "Edit Configuration".to_string(),
                            description: "Modify Commitaura's configuration settings".to_string(),
                            command: Command {
                                text: "commitaura config edit".to_string(),
                                description: "Edit configuration file".to_string(),
                                icon_path: "M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Add Templates".to_string(),
                            description: "Create custom commit message templates".to_string(),
                            command: Command {
                                text: "commitaura template add".to_string(),
                                description: "Add custom templates".to_string(),
                                icon_path: "M9 13h6m-3-3v6m-9 1V7a2 2 0 012-2h6l2 2h6a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2z".to_string(),
                            },
                        },
                        FlowStep {
                            title: "Configure Rules".to_string(),
                            description: "Set up custom rules for commit message generation".to_string(),
                            command: Command {
                                text: "commitaura rules set".to_string(),
                                description: "Configure commit rules".to_string(),
                                icon_path: "M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4".to_string(),
                            },
                        },
                    ],
                },
            ],
        ),
    ]
}

fn get_project_by_id(id: &str) -> Option<Project> {
    get_all_projects().into_iter().find(|p| p.id == id)
}

pub fn get_related_projects(current_id: &str) -> Vec<Project> {
    get_all_projects()
        .into_iter()
        .filter(|p| p.id != current_id)
        .take(3)
        .collect()
}
