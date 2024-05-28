use axum::{routing::get_service, Router};
use tower_http::services::ServeDir;


pub mod web;
pub mod ctx;
pub mod error;


pub fn routes_static() -> Router {
    let static_files_path = "./";
    tracing::info!("[routes_static] Serving static files from {}", static_files_path);
	Router::new().nest_service("/", get_service(ServeDir::new(static_files_path)))
}
