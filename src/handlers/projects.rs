use crate::{AppError, AppState};
use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse, Response},
};
use serde::Serialize;
use tera::Context;

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
pub struct UserFlow {
    title: String,
    content: String,
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
        user_flows: Vec<(&str, &str)>,
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
            user_flows: user_flows
                .into_iter()
                .map(|(title, content)| UserFlow {
                    title: title.to_string(),
                    content: content.to_string(),
                })
                .collect(),
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
                (
                    "Installation",
                    "1. Install Rust and Cargo\n2. Run `cargo install sagacity`\n3. Configure API keys\n4. Initialize project with `sagacity init`",
                ),
                (
                    "Basic Usage",
                    "Start exploring your codebase with natural language queries:\n- `sagacity search \"find all error handling patterns\"`\n- `sagacity analyze \"explain the authentication flow\"`\n- `sagacity context \"how does this relate to the user model?\"`",
                ),
                (
                    "Advanced Features",
                    "Learn about advanced features like custom indexing rules, context persistence, and integration with your development workflow. Includes examples of complex queries and automation scenarios.",
                ),
            ],
        ),
        Project::new(
            "commitaura",
            "Commitaura",
            "Rust / GPT-4",
            "Autonomous commit message generator using Git diff analysis and contextual understanding. Available on crates.io with intelligent change detection system.",
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
                (
                    "Setup & Configuration",
                    "Quick start guide:\n1. Install with `cargo install commitaura`\n2. Run `commitaura init` in your repository\n3. Configure your preferences in `.commitaura.toml`\n4. Set up Git hooks with `commitaura hooks install`",
                ),
                (
                    "Daily Workflow",
                    "Integrate Commitaura into your development workflow:\n- Stage your changes as usual\n- Use `commitaura suggest` to generate commit messages\n- Review and edit suggestions\n- Commit with `commitaura commit`",
                ),
                (
                    "Customization",
                    "Customize Commitaura to match your team's commit style:\n- Define custom commit message templates\n- Set up project-specific rules\n- Configure AI parameters\n- Integrate with CI/CD pipelines",
                ),
            ],
        ),
        Project::new(
            "resumatyk",
            "Resumatyk",
            "LaTeX / Python / Shell",
            "Powerful CLI tool for managing LaTeX resumes with AI-powered variant generation. Features email integration, OCR support, and intelligent content extraction.",
            "/images/resumatyk-demo.gif",
            "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z M14 2v6h6 M16 13H8 M16 17H8 M10 9H8",
            vec!["LaTeX", "Python", "Shell", "OCR"],
            vec![
                ("source", "https://github.com/yourusername/resumatyk"),
                ("documentation", "https://docs.resumatyk.dev"),
            ],
            vec![
                (
                    "AI Variant Generation",
                    "Creates tailored resume variants using AI understanding",
                    "M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15",
                ),
                (
                    "OCR Integration",
                    "Extract content from existing resumes and documents",
                    "M9 13h6m-3-3v6m5 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z",
                ),
                (
                    "Email Management",
                    "Integrated email tracking and application management",
                    "M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z",
                ),
            ],
            (
                "CLI-first design with modular architecture. Integrates LaTeX engine with Python processing.",
                "Custom LaTeX parser and template system. AI-powered content optimization and formatting.",
                "Maintaining consistent formatting across different LaTeX versions. Handling complex document structures.",
            ),
            vec!["Resume Builder", "AI Optimization", "Document Management", "Automation"],
            vec![
                (
                    "Initial Setup",
                    "Get started with Resumatyk:\n1. Install dependencies: LaTeX, Python 3.8+\n2. Install Resumatyk: `pip install resumatyk`\n3. Initialize workspace: `resumatyk init`\n4. Configure email settings: `resumatyk config email`",
                ),
                (
                    "Creating Your First Resume",
                    "Build your professional resume:\n1. Choose a template: `resumatyk template list`\n2. Create new resume: `resumatyk new my-resume`\n3. Edit content using your favorite editor\n4. Generate PDF: `resumatyk build my-resume`\n5. Create variants: `resumatyk variant create my-resume --role \"Software Engineer\"",
                ),
                (
                    "Managing Applications",
                    "Track your job applications:\n1. Add job posting: `resumatyk track add \"Company Name\"`\n2. Link resume variant: `resumatyk track link my-resume-variant1`\n3. Send application: `resumatyk apply --track-id 1`\n4. Check status: `resumatyk track status`\n5. Update application: `resumatyk track update 1 --status \"Interview\"",
                ),
                (
                    "Advanced Features",
                    "Leverage powerful features:\n- OCR import: `resumatyk import --ocr existing-resume.pdf`\n- AI optimization: `resumatyk optimize my-resume`\n- Bulk operations: `resumatyk batch process`\n- Custom templates: `resumatyk template create`\n- Analytics: `resumatyk stats view`",
                ),
            ],
        ),
        Project::new(
            "cybrnvim",
            "Cybrnvim",
            "Lua / Shell",
            "Custom NeoVim configuration focused on AI-enhanced development workflow. Features integrated tools, optimized keybindings, and efficient plugin management.",
            "/images/neovim-demo.gif",
            "M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3",
            vec!["Lua", "Shell", "NeoVim"],
            vec![
                ("source", "https://github.com/yourusername/cybrnvim"),
                ("documentation", "https://docs.cybrnvim.dev"),
            ],
            vec![
                (
                    "AI Integration",
                    "Seamless integration with AI coding assistants",
                    "M13 10V3L4 14h7v7l9-11h-7z",
                ),
                (
                    "Custom Keymaps",
                    "Ergonomic keyboard mappings for efficient coding",
                    "M12 4v1m6 11h2m-6 0h-2v4m0-11v3m0 0h.01M12 12h4.01M16 20h4M4 12h4m12 0h.01M5 8h2a1 1 0 001-1V5a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1zm12 0h2a1 1 0 001-1V5a1 1 0 00-1-1h-2a1 1 0 00-1 1v2a1 1 0 001 1zM5 20h2a1 1 0 001-1v-2a1 1 0 00-1-1H5a1 1 0 00-1 1v2a1 1 0 001 1z",
                ),
                (
                    "Plugin Management",
                    "Efficient plugin system with lazy loading",
                    "M4 7v10c0 2.21 3.582 4 8 4s8-1.79 8-4V7M4 7c0 2.21 3.582 4 8 4s8-1.79 8-4M4 7c0-2.21 3.582-4 8-4s8 1.79 8 4",
                ),
            ],
            (
                "Modular configuration system with lazy loading. Integrated with various AI services.",
                "Custom plugin management system. Optimized startup and runtime performance.",
                "Balancing functionality with performance. Managing plugin compatibility and updates.",
            ),
            vec!["IDE Evolution", "AI Enhancement", "Developer Tools", "Productivity"],
            vec![
                (
                    "Installation",
                    "Setting up Cybrnvim:\n1. Install NeoVim 0.9+\n2. Backup existing config: `mv ~/.config/nvim ~/.config/nvim.bak`\n3. Clone repository: `git clone https://github.com/yourusername/cybrnvim ~/.config/nvim`\n4. Install dependencies: `./install.sh`\n5. Start NeoVim and wait for initial setup",
                ),
                (
                    "Core Features",
                    "Essential Cybrnvim capabilities:\n- Space as leader key\n- Fuzzy finding: <leader>f\n- File explorer: <leader>e\n- Terminal toggle: <leader>t\n- AI completion: <C-space>\n- Quick commands: <leader>c\n- Project search: <leader>s",
                ),
                (
                 "AI Features",
                    "Leverage AI capabilities:\n- Code completion\n- Documentation generation\n- Code explanation\n- Refactoring suggestions\n- Bug detection\n- Natural language queries\n\nDetailed usage:\n1. Invoke completion: <C-space>\n2. Generate docs: <leader>ad\n3. Explain code: <leader>ae\n4. Suggest refactor: <leader>ar\n5. Check bugs: <leader>ab\n6. Ask question: <leader>aq",
                ),
                (
                    "Customization",
                    "Personalize your setup:\n1. Edit `lua/user/options.lua` for basic settings\n2. Modify `lua/user/keymaps.lua` for custom bindings\n3. Adjust `lua/user/plugins.lua` for plugin configuration\n4. Configure AI settings in `lua/user/ai.lua`\n5. Theme customization in `lua/user/colorscheme.lua`\n\nCommon customizations:\n- Adjust completion behavior\n- Configure language servers\n- Modify statusline\n- Set up custom snippets\n- Create project-specific settings",
                ),
                (
                    "Advanced Usage",
                    "Power user features:\n\nSnippets:\n- Create custom snippets in `lua/user/snippets`\n- Use snippet variables\n- Dynamic snippet expansion\n- Snippet conditions\n\nLSP Configuration:\n- Configure multiple LSPs\n- Custom diagnostics\n- Code actions\n- Symbol management\n\nDebugging:\n- DAP integration\n- Breakpoint management\n- Variable inspection\n- Call stack navigation\n\nGit Integration:\n- Advanced diffing\n- Blame visualization\n- Branch management\n- Interactive rebase\n\nProject Management:\n- Workspace configuration\n- Project-specific settings\n- Task automation\n- Build integration\n\nPerformance Optimization:\n- Plugin lazy loading\n- Startup optimization\n- Memory management\n- Cache configuration",
                ),
                (
                    "Plugin Management",
                    "Essential plugins and their configurations:\n\nCore Plugins:\n1. telescope.nvim - Fuzzy finder\n2. nvim-treesitter - Syntax highlighting\n3. mason.nvim - LSP installer\n4. nvim-lspconfig - LSP configuration\n5. nvim-cmp - Completion engine\n\nAI Plugins:\n1. copilot.lua - GitHub Copilot integration\n2. codeium.nvim - Codeium AI\n3. neural.nvim - Custom AI integration\n\nGit Plugins:\n1. gitsigns.nvim - Git integration\n2. neogit - Magit for Neovim\n\nUtility Plugins:\n1. which-key.nvim - Keybinding helper\n2. trouble.nvim - Diagnostic viewer\n3. nvim-dap - Debug adapter\n\nConfiguration Tips:\n- Use lazy loading\n- Configure dependencies\n- Manage conflicts\n- Update strategies",
                ),
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
