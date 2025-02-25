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

    let sagacity_installation_steps = vec![
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
    ];

    let sagacity_usage_steps = vec![
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
    ];

    let sagacity_architecture_steps = vec![
        FlowStep {
            title: "Core Components".to_string(),
            description: "Review the core architectural components".to_string(),
            command: Command {
                text: "tree src/".to_string(),
                description: "View project structure".to_string(),
                icon_path:
                    "M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                        .to_string(),
            },
        },
        FlowStep {
            title: "API Integration".to_string(),
            description: "Review the Claude API integration".to_string(),
            command: Command {
                text: "cat src/api.rs".to_string(),
                description: "View API integration code".to_string(),
                icon_path: "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4".to_string(),
            },
        },
    ];

    let commitaura_setup_steps = vec![
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
            title: "Add API Key".to_string(),
            description: "Add your Anthropic API key to your shell environment".to_string(),
            command: Command {
                text: "echo 'export ANTHROPIC_API_KEY=your-api-key-here' >> ~/.zshrc && source ~/.zshrc".to_string(),
                description: "Configure API key".to_string(),
                icon_path: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z".to_string(),
            },
        },
    ];

    let commitaura_workflow_steps = vec![
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
                text: "commitaura".to_string(),
                description: "Generate commit message".to_string(),
                icon_path: "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z".to_string(),
            },
        },
        FlowStep {
            title: "Commit Changes".to_string(),
            description: "Commit your changes with the generated message".to_string(),
            command: Command {
                text: "y/n".to_string(),
                description: "Confirm commit message".to_string(),
                icon_path: "M5 13l4 4L19 7".to_string(),
            },
        },
    ];

    Ok(vec![
        Project::new(
            "sagacity",
            "Sagacity",
            "Rust / Claude API",
            "Intelligent codebase exploration tool using natural language interaction. Features novel summarizational indexing system and persistent context memory.",
            "/static/images/sagacity.jpg",
            "Intelligent codebase exploration tool powered by Claude API",
            "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2 M12 3a4 4 0 1 0 0 8 4 4 0 0 0 0-8z",
            vec!["Rust", "Claude API", "NLP", "Git"],
            vec![
                ("crates.io", "https://crates.io/crates/sagacity"),
                ("github", "https://github.com/cybrdelic/sagacity"),
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
                    "M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253",
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
            sagacity_flow_configs,
        )?,
        Project::new(
            "commitaura",
            "Commitaura",
            "AI-Powered Commit Message Generator",
            "Autonomously generate commit messages for your staged commits, using diff analysis as contextualization.",
            "/static/images/commitaura.gif",
            "AI-powered commit message generation in action",
            "M6 3v12 M18 6a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M6 18a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M18 9a9 9 0 0 1-9 9",
            vec!["Rust", "Claude API", "Git"],
            vec![
                ("View Documentation", "https://docs.commitaura.dev"),
                ("View Source", "https://github.com/cybrdelic/commitaura"),
            ],
            vec![
                (
                    "Claude Integration",
                    "Analyzes git diffs using the Claude API to understand code changes contextually",
                    "M8 7v8a2 2 0 002 2h6M8 7V5a2 2 0 012-2h4.586a1 1 0 01.707.293l4.414 4.414a1 1 0 01.293.707V15a2 2 0 01-2 2h-2",
                ),
                (
                    "Smart Commit Messages",
                    "Generates meaningful commit messages using context-aware analysis",
                    "M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z",
                ),
            ],
            (
                "Event-driven architecture with Git hook integration. Uses Rust async runtime for performance.",
                "Implements custom diff parsing and Claude prompt engineering. Features caching and rate limiting.",
                "Handling complex Git histories and merge commits. Ensuring consistent message quality.",
            ),
            vec!["Git Integration", "AI-Powered", "Developer Workflow", "Productivity"],
            vec![
                ("setup".to_string(), commitaura_setup_steps),
                ("workflow".to_string(), commitaura_workflow_steps),
            ],
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
