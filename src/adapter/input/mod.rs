use axum::{http::{Method, Uri}, response::{IntoResponse, Response}, routing::get_service, Json, Router};
use serde_json::json;
use uuid::Uuid;
use tower_http::services::ServeDir;

use crate::config::log::log_request;

pub mod web;
pub mod ctx;
pub mod error;

pub use self::error::{Result, Error};

pub fn routes_static() -> Router {
    let static_files_path = "./";
    tracing::info!("[routes_static] Serving static files from {}", static_files_path);
	Router::new().nest_service("/", get_service(ServeDir::new(static_files_path)))
}
