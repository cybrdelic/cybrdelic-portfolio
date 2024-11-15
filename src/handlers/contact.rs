use axum::{
    extract::Form,
    response::Json,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[derive(Serialize)]
pub struct ContactResponse {
    success: bool,
    message: String,
    timestamp: String,
}

pub async fn submit(Form(form): Form<ContactForm>) -> Json<ContactResponse> {
    Json(ContactResponse {
        success: true,
        message: format!("Message received from {}. Establishing secure connection...", form.name),
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}
