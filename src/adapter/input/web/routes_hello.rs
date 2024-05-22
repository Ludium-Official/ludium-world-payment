use axum::{extract::Query, response::{Html, IntoResponse}, routing::get, Router};
use serde::Deserialize;


#[derive(Debug, Deserialize)]
struct HelloParams {
	name: Option<String>,
}

// region:    --- Routes Hello

pub fn routes() -> Router {
	Router::new()
		.route("/hello", get(handler_hello))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
	tracing::debug!("[handler] handler_hello - {params:?}");

	let name = params.name.as_deref().unwrap_or("World!");
	Html(format!("Hello <strong>{name}</strong>"))
}

// endregion: --- Routes Hello
