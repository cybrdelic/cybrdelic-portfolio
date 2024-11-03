// src/models/project.rs
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub long_description: String,
    pub technologies: Vec<String>,
    pub github_url: String,
    pub features: Vec<String>,
    pub timeline: String,
    pub impact: String,
    pub tech_stack_details: String,
    pub challenges: String,
    pub future_plans: String,
    pub preview_image: String,
}

