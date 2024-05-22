use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::trace::TraceLayer;
use tracing::Level;

use std::time::Duration;
use axum::body::Body;
use axum::extract::Request;
use axum::response::Response;

use tracing_subscriber::filter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use axum::http::{Method, Uri};
use serde::Serialize;
use serde_json::{json, Value};
use serde_with::skip_serializing_none;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use crate::adapter::input::ctx::Ctx;
use crate::adapter::input::error::{ClientError, Result, Error};


pub fn init_tracing() {
    let tracing_layer = tracing_subscriber::fmt::layer();

    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::DEBUG)
        .with_target("tower_http::trace::on_request", Level::DEBUG)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::DEBUG);

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .init();
}
    
pub fn request_logging_middleware() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>, impl Fn(&Request<Body>) -> tracing::Span + Clone, impl Fn(&Request<Body>, &tracing::Span) + Clone, impl Fn(&Response, Duration, &tracing::Span) + Clone> {
    TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            tracing::info_span!("request", method = %request.method(), uri = %request.uri())
        })
        .on_request(|request: &Request<Body>, _: &tracing::Span| {
            tracing::info!("Starting request to {}", request.uri());
        })
        .on_response(|response: &Response, latency: Duration, _: &tracing::Span| {
            tracing::info!("Response status: {}, took: {:?}", response.status(), latency);
        })
}

#[skip_serializing_none]
#[derive(Serialize)]
struct RequestLogLine {
	uuid: String,      // uuid string formatted
	timestamp: String, // (should be iso8601)

	// -- User and context attributes.
	user_id: Option<u64>,

	// -- http request attributes.
	req_path: String,
	req_method: String,

	// -- Errors attributes.
	client_error_type: Option<String>,
	error_type: Option<String>,
	error_data: Option<Value>,
}

pub async fn log_request(
	uuid: Uuid,
	req_method: Method,
	uri: Uri,
	ctx: Option<Ctx>,
	service_error: Option<&Error>,
	client_error: Option<ClientError>,
) -> Result<()> {
	let timestamp = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_millis();

	let error_type = service_error.map(|se| se.to_string());
	let error_data = serde_json::to_value(service_error)
		.ok()
		.and_then(|mut v| v.get_mut("data").map(|v| v.take()));

	let log_line = RequestLogLine {
		uuid: uuid.to_string(),
		timestamp: timestamp.to_string(),

		req_path: uri.to_string(),
		req_method: req_method.to_string(),

		user_id: ctx.map(|c| c.user_id()),

		client_error_type: client_error.map(|e| e.as_ref().to_string()),

		error_type,
		error_data,
	};

	tracing::debug!("[request]\n{}", json!(log_line));
	// TODO - Send to cloud-watch.

	Ok(())
}
