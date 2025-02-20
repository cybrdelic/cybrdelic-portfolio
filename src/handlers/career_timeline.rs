// src/handlers/career_timeline.rs

use serde::Serialize;
use std::io;

#[derive(Serialize, Clone)]
pub struct Topic {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Clone)]
pub struct TimelineEvent {
    pub title: String,
    pub description: String,
    pub location: String,
    pub start_date: String,
    pub end_date: String,
    pub company_name: String,
    pub topics: Vec<Topic>,
}

pub fn get_career_timeline() -> Result<Vec<TimelineEvent>, io::Error> {
    let timeline = vec![
        TimelineEvent {
            title: "Started as Junior Developer".to_string(),
            description: "Joined ABC Corp as a junior developer focused on full stack web development.".to_string(),
            location: "New York, NY".to_string(),
            start_date: "January 2021".to_string(),
            end_date: "December 2021".to_string(),
            company_name: "ABC Corp".to_string(),
            topics: vec![
                Topic {
                    name: "Full Stack Web Development".to_string(),
                    description: "Gained comprehensive skills in both frontend and backend technologies, implementing responsive designs and scalable services.".to_string(),
                },
                Topic {
                    name: "CRUD API Development".to_string(),
                    description: "Built and maintained robust API endpoints to facilitate seamless data operations across applications.".to_string(),
                },
                Topic {
                    name: "Complex Forms and Fields".to_string(),
                    description: "Developed dynamic forms with real-time validations and adaptive field management.".to_string(),
                },
            ],
        },
        TimelineEvent {
            title: "Promoted to Software Engineer".to_string(),
            description: "Expanded role to include internal tooling and automation projects.".to_string(),
            location: "New York, NY".to_string(),
            start_date: "January 2022".to_string(),
            end_date: "December 2022".to_string(),
            company_name: "ABC Corp".to_string(),
            topics: vec![
                Topic {
                    name: "Cross-Service Authentication".to_string(),
                    description: "Implemented secure authentication mechanisms across distributed microservices.".to_string(),
                },
                Topic {
                    name: "Internal Developer CLI".to_string(),
                    description: "Created command-line tools to streamline development workflows and internal processes.".to_string(),
                },
                Topic {
                    name: "Role Based Permissions".to_string(),
                    description: "Designed systems to manage and enforce user permissions effectively across platforms.".to_string(),
                },
            ],
        },
        TimelineEvent {
            title: "Launched tn-cli".to_string(),
            description: "Developed tn-cli, a command-line tool for cross-service function calling and user flow orchestration.".to_string(),
            location: "Remote".to_string(),
            start_date: "March 2023".to_string(),
            end_date: "Present".to_string(),
            company_name: "ABC Corp".to_string(),
            topics: vec![
                Topic {
                    name: "Automation".to_string(),
                    description: "Streamlined repetitive tasks by automating critical development processes.".to_string(),
                },
                Topic {
                    name: "CLI/TUI Development".to_string(),
                    description: "Built interactive terminal interfaces to improve efficiency and user experience.".to_string(),
                },
                Topic {
                    name: "User Flow Orchestration".to_string(),
                    description: "Coordinated complex user interactions and processes seamlessly across services.".to_string(),
                },
            ],
        },
        TimelineEvent {
            title: "Pioneered AI-Driven Projects".to_string(),
            description: "Initiated innovative projects like myriad.ai and commitaura, integrating AI to streamline code analysis and commit messaging.".to_string(),
            location: "San Francisco, CA".to_string(),
            start_date: "January 2024".to_string(),
            end_date: "Present".to_string(),
            company_name: "XYZ Innovations".to_string(),
            topics: vec![
                Topic {
                    name: "AI Integration".to_string(),
                    description: "Leveraged artificial intelligence to enhance functionality and drive innovation in internal tools.".to_string(),
                },
                Topic {
                    name: "Codebase Contextualization".to_string(),
                    description: "Implemented systems that provide contextual insights into codebases, reducing onboarding time.".to_string(),
                },
                Topic {
                    name: "AI-Powered Commit Messages".to_string(),
                    description: "Developed tools that generate meaningful commit messages by analyzing code changes with AI.".to_string(),
                },
            ],
        },
    ];

    Ok(timeline)
}
