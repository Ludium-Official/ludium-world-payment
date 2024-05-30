use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

pub fn routes() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}
