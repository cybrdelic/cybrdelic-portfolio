// src/bin/deploy.rs
use std::process::{Command, ExitStatus};
use std::path::Path;
use std::fs;
use std::env;
use std::io;

fn run_command(command: &str, args: &[&str]) -> io::Result<ExitStatus> {
    println!("Running: {} {}", command, args.join(" "));
    Command::new(command)
        .args(args)
        .status()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting deployment process...");

    // Step 1: Build the static site
    println!("📦 Building static site...");
    let build_status = run_command("cargo", &["run", "--bin", "build-site"])?;
    if !build_status.success() {
        return Err("Failed to build static site".into());
    }
    println!("✅ Build completed successfully!");

    // Step 2: Create a temporary directory and copy dist contents
    let temp_dir = env::temp_dir().join("cybrdelic_deploy");
    println!("📁 Using temporary directory: {:?}", temp_dir);
    
    if temp_dir.exists() {
        fs::remove_dir_all(&temp_dir)?;
    }
    fs::create_dir_all(&temp_dir)?;

    // Copy dist contents to the temp directory
    copy_dir_recursive(Path::new("dist"), &temp_dir)?;
    println!("📋 Copied dist contents to temporary directory");

    // Step 3: Get current branch and prepare for gh-pages
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;
    let current_branch = String::from_utf8(output.stdout)?.trim().to_string();
    println!("📝 Current branch: {}", current_branch);

    // Check if gh-pages branch exists
    let gh_pages_exists = Command::new("git")
        .args(["show-ref", "--verify", "--quiet", "refs/heads/gh-pages"])
        .status()?
        .success();

    if gh_pages_exists {
        println!("🔍 Found existing gh-pages branch");
        run_command("git", &["checkout", "gh-pages"])?;
    } else {
        println!("🌱 Creating new gh-pages branch");
        run_command("git", &["checkout", "--orphan", "gh-pages"])?;
        run_command("git", &["reset", "--hard"])?;
        run_command("git", &["commit", "--allow-empty", "-m", "Initialize gh-pages branch"])?;
        run_command("git", &["checkout", &current_branch])?;
        run_command("git", &["checkout", "gh-pages"])?;
    }

    // Step 4: Clean the gh-pages branch and copy new files
    println!("🧹 Cleaning gh-pages branch...");
    // Get a list of all entries except .git and .
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        let name = path.file_name().unwrap().to_string_lossy();
        
        if name != ".git" && name != "." && name != ".." {
            if path.is_dir() {
                fs::remove_dir_all(path)?;
            } else {
                fs::remove_file(path)?;
            }
        }
    }

    // Copy new files from temp directory
    println!("📋 Copying new files from temporary directory...");
    copy_dir_recursive(&temp_dir, Path::new("."))?;

    // Step 5: Commit and push changes to gh-pages branch ONLY
    println!("💾 Committing changes to gh-pages branch...");
    run_command("git", &["add", "."])?;
    
    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let commit_message = format!("Deploy portfolio to GitHub Pages on {}", date);
    
    run_command("git", &["commit", "-m", &commit_message])?;

    println!("☁️ Pushing to GitHub (gh-pages branch only)...");
    run_command("git", &["push", "-f", "origin", "gh-pages"])?;

    // Step 6: Clean up and switch back to original branch WITHOUT merging
    println!("🧹 Cleaning up and returning to source branch...");
    fs::remove_dir_all(&temp_dir)?;
    run_command("git", &["checkout", &current_branch])?;
    
    println!("⚠️ IMPORTANT: The gh-pages branch contains only built files.");
    println!("   DO NOT merge gh-pages back into your source branch!");

    println!("✨ Deployment completed successfully!");
    println!("🌐 Your site is now available at: https://YOUR_USERNAME.github.io/cybrdelic-portfolio/");
    println!("   (Replace YOUR_USERNAME with your GitHub username)");

    Ok(())
}

fn copy_dir_recursive(source: &Path, destination: &Path) -> io::Result<()> {
    if !source.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Source is not a directory"));
    }

    if !destination.exists() {
        fs::create_dir_all(destination)?;
    }

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name = entry.file_name();
        let dest_path = destination.join(file_name);

        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}