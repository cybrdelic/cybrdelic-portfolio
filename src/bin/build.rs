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

    render_index(&tera, &projects)?;
    render_career_timeline(&tera, &career_timeline)?;
    render_project_details(&tera, &projects)?;
    copy_static_assets()?;

    println!("Build completed successfully!");
    Ok(())
}

fn render_index(tera: &Tera, projects: &[Project]) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("projects", projects);

    let rendered = tera.render("sections/projects.html", &context)?;
    let output_path = Path::new("dist").join("index.html");
    fs::create_dir_all(output_path.parent().unwrap())?;
    fs::write(&output_path, rendered)?;
    println!("Rendered index page: index.html");
    Ok(())
}

fn render_career_timeline(
    tera: &Tera,
    timeline: &[cybrdelic_portfolio::handlers::career_timeline::TimelineEvent],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut context = Context::new();
    context.insert("timeline", timeline);

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
) -> Result<(), Box<dyn std::error::Error>> {
    for project in projects {
        let mut context = Context::new();
        context.insert("project", project);
        context.insert("related_projects", &get_related_projects(&project.id));
        context.insert("preload_hint", &true);

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
    fs::create_dir_all("dist/static")?;
    for entry in glob("static/**/*")? {
        let path = entry?;
        let dest = Path::new("dist").join(path.strip_prefix("static/")?);
        if path.is_file() {
            fs::create_dir_all(dest.parent().unwrap())?;
            fs::copy(&path, &dest)?;
            println!("Copied static asset: {:?}", dest);
        }
    }
    Ok(())
}
