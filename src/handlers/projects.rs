// src/handlers/projects.rs
use crate::models::Project;
use axum::{extract::Path, response::Html, Json};

pub async fn serve_index() -> Html<&'static str> {
    Html(include_str!("../../static/index.html"))
}

pub async fn serve_project_page(Path(id): Path<String>) -> Html<String> {
    let project = get_project_by_id(&id);
    let template = include_str!("../../static/project-template.html");

    Html(
        template
            .replace("{{id}}", &project.id)
            .replace("{{title}}", &project.title)
            .replace("{{description}}", &project.description)
            .replace("{{long_description}}", &project.long_description)
            .replace(
                "{{technologies}}",
                &project
                    .technologies
                    .iter()
                    .map(|tech| format!(r#"<span class="tech-tag">{}</span>"#, tech))
                    .collect::<Vec<_>>()
                    .join(""),
            )
            .replace("{{github_url}}", &project.github_url)
            .replace(
                "{{features}}",
                &project
                    .features
                    .iter()
                    .map(|f| format!("<li>{}</li>", f))
                    .collect::<Vec<_>>()
                    .join("\n"),
            )
            .replace("{{timeline}}", &project.timeline)
            .replace("{{impact}}", &project.impact)
            .replace("{{tech_stack_details}}", &project.tech_stack_details)
            .replace("{{challenges}}", &project.challenges)
            .replace("{{future_plans}}", &project.future_plans),
    )
}

pub async fn get_projects() -> Html<String> {
    let projects = get_projects_data();
    let mut html = String::new();

    for project in projects {
        html.push_str(&format!(
            r#"
            <div class="project-card">
                <div class="terminal-header">
                    <span class="terminal-title">[{}]</span>
                </div>
                <div class="project-content">
                    <h3>{}</h3>
                    <p class="project-desc">{}</p>
                    <div class="tech-tags">
                        {}
                    </div>
                    <div class="project-actions">
                        <a href="/project/{}" class="cyber-button">explore_project()</a>
                        <a href="{}" class="cyber-button" target="_blank">view_source()</a>
                    </div>
                </div>
            </div>
            "#,
            project.id,
            project.title,
            project.description,
            project
                .technologies
                .iter()
                .map(|tech| format!(r#"<span class="tech-tag">{}</span>"#, tech))
                .collect::<Vec<_>>()
                .join(""),
            project.id,
            project.github_url
        ));
    }

    Html(html)
}

pub async fn get_project_details(Path(id): Path<String>) -> Json<Project> {
    Json(get_project_by_id(&id))
}

fn get_project_by_id(id: &str) -> Project {
    let projects = get_projects_data();
    projects
        .into_iter()
        .find(|p| p.id == id)
        .unwrap_or_else(|| Project {
            id: "not-found".to_string(),
            title: "Project Not Found".to_string(),
            description: "This project does not exist.".to_string(),
            long_description: "".to_string(),
            technologies: vec![],
            github_url: "".to_string(),
            features: vec![],
            timeline: "".to_string(),
            impact: "".to_string(),
            tech_stack_details: "".to_string(),
            challenges: "".to_string(),
            future_plans: "".to_string(),
            preview_image: "".to_string(),
        })
}

fn get_projects_data() -> Vec<Project> {
    vec![
        Project {
            id: "commitaura".to_string(),
            title: "Commitaura".to_string(),
            description: "AI-powered commit message generator using OpenAI's GPT API".to_string(),
            long_description: "Commitaura revolutionizes the way developers write commit messages by leveraging OpenAI's GPT to analyze code changes and generate meaningful, contextual commit messages automatically.".to_string(),
            technologies: vec!["Rust".to_string(), "OpenAI".to_string(), "CLI".to_string(), "Git".to_string()],
            github_url: "https://github.com/cybrdelic/commitaura".to_string(),
            features: vec![
                "Intelligent commit analysis".to_string(),
                "Custom commit message templates".to_string(),
                "Git hook integration".to_string(),
                "Multi-language support".to_string(),
            ],
            timeline: "Started development in 2023, released v1.0 in early 2024".to_string(),
            impact: "Reduced commit message writing time by 70% while improving consistency and clarity across team communications.".to_string(),
            tech_stack_details: "Built with Rust for performance, integrates with OpenAI's GPT-4 API, uses Git2-rs for repository interaction.".to_string(),
            challenges: "Optimizing token usage while maintaining message quality, handling large diffs efficiently.".to_string(),
            future_plans: "Adding support for commit convention detection, team-specific templates, and multilingual commit messages.".to_string(),
            preview_image: "/static/images/commitaura-preview.png".to_string(),
        },
        Project {
            id: "sagacity".to_string(),
            title: "Sagacity".to_string(),
            description: "Codebase exploration tool powered by Claude AI".to_string(),
            long_description: "Sagacity is an intelligent codebase exploration tool that uses Claude AI to help developers understand complex codebases quickly and efficiently.".to_string(),
            technologies: vec!["Rust".to_string(), "Claude AI".to_string(), "CLI".to_string(), "AST Parsing".to_string()],
            github_url: "https://github.com/cybrdelic/sagacity".to_string(),
            features: vec![
                "Natural language code queries".to_string(),
                "Semantic code search".to_string(),
                "Interactive documentation generation".to_string(),
                "Code pattern recognition".to_string(),
            ],
            timeline: "Development began Q4 2023, beta release Q1 2024".to_string(),
            impact: "Reduced new developer onboarding time by 50% in test implementations.".to_string(),
            tech_stack_details: "Rust core with advanced AST parsing, Claude AI integration, and a custom query engine.".to_string(),
            challenges: "Developing accurate code context gathering, optimizing large codebase analysis.".to_string(),
            future_plans: "Adding collaborative features, IDE integration, and custom training capabilities.".to_string(),
            preview_image: "/static/images/sagacity-preview.png".to_string(),
        },
    ]
}
