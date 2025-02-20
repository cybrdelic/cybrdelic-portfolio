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
            title: "Junior Developer".to_string(),
            description: "Role at ABC Corp with emphasis on full stack web development.".to_string(),
            location: "New York, NY".to_string(),
            start_date: "January 2021".to_string(),
            end_date: "December 2021".to_string(),
            company_name: "ABC Corp".to_string(),
            topics: vec![
                Topic {
                    name: "Full Stack Web Development".to_string(),
                    description: "Comprehensive skill set in both frontend and backend technologies, with responsive designs and scalable services.".to_string(),
                },
                Topic {
                    name: "CRUD API Development".to_string(),
                    description: "Robust endpoints for seamless data operations across applications.".to_string(),
                },
                Topic {
                    name: "Complex Forms and Fields".to_string(),
                    description: "Dynamic forms with real-time validations and adaptive field management.".to_string(),
                },
            ],
        },
        TimelineEvent {
            title: "Software Engineer".to_string(),
            description: "Expanded responsibilities for internal tooling and automation projects at ABC Corp.".to_string(),
            location: "New York, NY".to_string(),
            start_date: "January 2022".to_string(),
            end_date: "December 2022".to_string(),
            company_name: "ABC Corp".to_string(),
            topics: vec![
                Topic {
                    name: "Cross-Service Authentication".to_string(),
                    description: "Secure authentication mechanisms across distributed microservices.".to_string(),
                },
                Topic {
                    name: "Internal Developer CLI".to_string(),
                    description: "Command-line tools that streamline development workflows and internal processes.".to_string(),
                },
                Topic {
                    name: "Role Based Permissions".to_string(),
                    description: "Systems for user permission management across platforms.".to_string(),
                },
            ],
        },
    ];

    Ok(timeline)
}
