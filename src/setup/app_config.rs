// src/setup/app_config.rs
use crate::routes;
use axum::Router;

pub fn create_router() -> Router {
    routes::create_routes()
}
