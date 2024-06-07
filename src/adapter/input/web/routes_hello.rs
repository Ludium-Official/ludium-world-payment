use axum::{response::{Html, IntoResponse}, routing::get, Router};


pub fn routes() -> Router {
	Router::new()
		.route("/hello", get(hello))
}

#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "hello")
    ),
	tag = "Hello"
)]
pub async fn hello() -> impl IntoResponse {
	Html("Hello, I'm payment!")
}
