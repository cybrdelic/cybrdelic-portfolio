// src/bin/build.rs
use cybrdelic_portfolio::handlers::career_timeline::get_career_timeline;
use cybrdelic_portfolio::handlers::projects::{get_all_projects, get_related_projects, Project};
use glob::glob;
use std::fs;
use std::path::Path;
use tera::{Context, Tera};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tera = Tera::new("templates/**/*")?;
    fs::create_dir_all("dist")?;

    let projects = get_all_projects()?;
    let career_timeline = get_career_timeline()?;

    // Define the base path for GitHub Pages
    // Use empty string for local development, "/cybrdelic-portfolio/" for GitHub Pages
    let base_path = "/cybrdelic-portfolio/"; // Change as needed

    render_index(&tera, &projects, &career_timeline, base_path)?;
    render_career_timeline(&tera, &career_timeline, base_path)?;
    render_project_details(&tera, &projects, base_path)?;
    copy_static_assets()?;

    println!("Build completed successfully with base path: {}", base_path);
    Ok(())
}

fn render_index(
    tera: &Tera,
    projects: &[Project],
    timeline: &[cybrdelic_portfolio::handlers::career_timeline::TimelineEvent],
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("projects", projects);
    context.insert("timeline", timeline);
    context.insert("career_timeline", timeline);
    context.insert("base_path", base_path);

    let rendered = tera.render("index.html", &context)?;
    let output_path = Path::new("dist").join("index.html");
    fs::create_dir_all(output_path.parent().unwrap())?;
    fs::write(&output_path, rendered)?;
    println!("Rendered index page: index.html");
    Ok(())
}

fn render_career_timeline(
    tera: &Tera,
    timeline: &[cybrdelic_portfolio::handlers::career_timeline::TimelineEvent],
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("career_timeline", timeline);
    context.insert("base_path", base_path);

    let template_name = "career_timeline.html";
    if tera.get_template(template_name).is_ok() {
        let rendered = tera.render(template_name, &context)?;
        let output_path = Path::new("dist").join("career.html");
        fs::create_dir_all(output_path.parent().unwrap())?;
        fs::write(&output_path, rendered)?;
        println!("Rendered career timeline page: career.html");
    } else {
        println!(
            "Warning: Template '{}' not found, skipping career timeline.",
            template_name
        );
    }
    Ok(())
}

fn render_project_details(
    tera: &Tera,
    projects: &[Project],
    base_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for project in projects {
        let mut context = Context::new();
        context.insert("project", project);
        context.insert("related_projects", &get_related_projects(&project.id));
        context.insert("preload_hint", &true);
        context.insert("base_path", base_path);

        let rendered = tera.render("project_detail.html", &context)?;
        let output_path = Path::new("dist")
            .join("projects")
            .join(&project.id)
            .join("index.html");
        fs::create_dir_all(output_path.parent().unwrap())?;
        fs::write(&output_path, rendered)?;
        println!(
            "Rendered project detail page: projects/{}/index.html",
            project.id
        );
    }
    Ok(())
}

fn copy_static_assets() -> Result<(), Box<dyn std::error::Error>> {
    // Ensure all necessary directories exist
    fs::create_dir_all("dist/static/css")?;
    fs::create_dir_all("dist/static/js")?;
    fs::create_dir_all("dist/static/images")?;
    fs::create_dir_all("dist/static/fonts")?;

    // Copy static assets
    let patterns = [
        "static/**/*",
        "css/**/*",
        "assets/**/*",
        "images/**/*",
        "js/**/*",
        "fonts/**/*",
    ];

    for pattern in patterns.iter() {
        for entry in glob(pattern)? {
            match entry {
                Ok(path) => {
                    // Determine the destination path
                    let dest = if path.starts_with("static/") {
                        // If it's already in static/, maintain the path structure
                        Path::new("dist").join(&path)
                    } else {
                        // Otherwise, put it under static/
                        Path::new("dist/static").join(&path)
                    };

                    if path.is_file() {
                        fs::create_dir_all(dest.parent().unwrap())?;
                        match fs::copy(&path, &dest) {
                            Ok(_) => println!("Copied asset: {:?} -> {:?}", path, dest),
                            Err(e) => println!("Failed to copy {:?}: {}", path, e),
                        }
                    }
                }
                Err(e) => println!("Error in glob pattern {}: {}", pattern, e),
            }
        }
    }

    // Handle individual files in the root that need to be copied
    let root_files = [
        "styles.css",
        "favicon.ico",
        "robots.txt",
        "site.webmanifest",
    ];
    for file in root_files.iter() {
        let path = Path::new(file);
        if path.exists() && path.is_file() {
            fs::copy(path, Path::new("dist").join(path))?;
            println!("Copied root file: {:?}", path);
        }
    }

    println!("Static assets copied successfully");
    Ok(())
}
