use axum::{http::{Method, Uri}, response::{IntoResponse, Response}, routing::get_service, Json, Router};
use serde_json::json;
use uuid::Uuid;
use tower_http::services::ServeDir;

use crate::{adapter::input::{ctx::Ctx, error::Error}, config::log::log_request};


pub async fn mapper(
	ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	tracing::debug!("[response_mapper] main_response_mapper");
	let uuid = Uuid::new_v4();

	let service_error = res.extensions().get::<Error>();
	let client_status_error = service_error.map(|se| se.client_status_and_error());

	// TODO: imporve this unwrap. use: client_status_and_error
	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error)| {
				let client_error_body = json!({
					"error": {
						"type": client_error.as_ref(),
						"req_uuid": uuid.to_string(),
					}
				});

                tracing::debug!("[client_error_body] {client_error_body}");
				(*status_code, Json(client_error_body)).into_response()
			});

	let client_error = client_status_error.unzip().1;

	// TODO: Need to hander if log_request fail (but should not fail request)
	let _ =
		log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

	error_response.unwrap_or(res)
}

