use std::sync::Arc;

use axum::{body::{Body, Bytes}, http::{Method, Uri}, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::{adapter::input::{ctx::Ctx, error::Error}, config::log::log_request};

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse{
	pub status_code: u16,
	pub message: String,
}

pub async fn mapper(
	ctx: Option<Ctx>,
	uri: Uri,
	req_method: Method,
	res: Response,
) -> Response {
	let uuid = Uuid::new_v4();

	let service_error = res.extensions().get::<Arc<Error>>().map(Arc::as_ref).cloned();
	let client_status_error = service_error.clone().map(|se| se.client_status_and_error());

	let error_response =
		client_status_error
			.as_ref()
			.map(|(status_code, client_error_message)| {
				let error_response = ErrorResponse {
					status_code: status_code.as_u16(),
					message: client_error_message.to_string(),
				};

				(*status_code, Json(error_response)).into_response()
			});

	let client_error_message = client_status_error.unzip().1;

	let (parts, body) = res.into_parts();
	let res_body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(e) => {
            tracing::info!("Failed to read body: {:?}", e);
            Bytes::new()
        }
    };
	let res_body_str = String::from_utf8_lossy(&res_body_bytes).to_string();

	let _ =
		log_request(uuid, req_method, uri, ctx, 
			service_error.as_ref(), 
			client_error_message.clone().as_ref(),
			Some(&res_body_str)).await;

	error_response.unwrap_or_else(|| Response::from_parts(parts, Body::from(res_body_bytes)))
}

