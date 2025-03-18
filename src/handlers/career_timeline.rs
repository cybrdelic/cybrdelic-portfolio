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
            title: "Associate Software Developer".to_string(),
            description: "Remote full-stack web development role at TalentNow with emphasis on Talent Management".to_string(),
            location: "Remote".to_string(),
            start_date: "December 2021".to_string(),
            end_date: "Feb 2025".to_string(),
            company_name: "TalentNow".to_string(),
            topics: vec![
                Topic {
                    name: "Full Stack Web Development".to_string(),
                    description: "Maintained and iterated on a full-stack, multi-tenant, microservice enterprise web application".to_string(),
                },
                Topic {
                    name: "CRUD API Development".to_string(),
                    description: "Robust endpoints for seamless data operations across applications.".to_string(),
                },
                Topic {
                    name: "Complex Forms and Fields".to_string(),
                    description: "Dynamic forms with real-time validations and adaptive field management.".to_string(),
                },
                Topic {
                    name: "Highly Interactive Tables".to_string(),
                    description: "Developed and maintained various tables with a myriad of filtering, sorting, and quick action requirements.".to_string()
                }
            ],
        },
        TimelineEvent {
            title: "Software Engineer".to_string(),
            description: "Expanded responsibilities for internal tooling and automation projects at TalentNow.".to_string(),
            location: "Remote".to_string(),
            start_date: "Feb 2025".to_string(),
            end_date: "Current".to_string(),
            company_name: "TalentNow".to_string(),
            topics: vec![
                Topic {
                    name: "Cross-Service Authentication".to_string(),
                    description: "Implemented secure authentication mechanisms across distributed microservices, in various environments. ".to_string(),
                },
                Topic {
                    name: "Internal Developer CLI".to_string(),
                    description: " Took initiative to create an internal developer command-line tool to automate user flows during development, increasing developer productivity.".to_string(),
                },
                Topic {
                    name: "Role Based Permissions".to_string(),
                    description: "Implemented systems for user permission management across the entire platform and a plethora of user roles.".to_string(),
                },
                Topic {
                    name: "Email and In-App Notification Systems".to_string(),
                    description: "Took ownership over the project's email and in-app notifications system, sprawling across over 10 microservices, over 20 user types with different permissions and contextual requirements.".to_string(),
                }
            ],
        },
    ];

    Ok(timeline)
}
