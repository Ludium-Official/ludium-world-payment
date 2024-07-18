use std::sync::Arc;

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use serde_with::serde_as;

use crate::{adapter::output::{self, near, persistence::db}, usecase};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    // -- Auth
    AuthFailNoAuthInformation,
    AuthFailCtxNotInRequestExt,
	Unauthorized { message: String },

	// -- Request Params
	UUIDParsingError { message: String },
	
	// -- Output
	Postgres(db::error::Error),
	Near(near::error::Error),

	// -- Usecase 
	Usecase(usecase::error::Error),

	// -- 
	Unknown
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		tracing::debug!("[into_response] - {self:?}");

		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
		response.extensions_mut().insert(Arc::new(self));
		response
	}
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, String) {
		#[allow(unreachable_patterns)]
		match self {
			// -- Auth 
			Self::AuthFailNoAuthInformation
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, "No Auth Information".to_string())
			},
			Self::Unauthorized { message } => (
				StatusCode::FORBIDDEN,
				message.to_string(),
			),

			// -- Request Params
			Self::UUIDParsingError { message } => (
				StatusCode::BAD_REQUEST,
				message.to_string(),
			),
			// -- Output
			Self::Postgres(error) => error.client_status_and_error(),
			Self::Near(error) => error.client_status_and_error(),

			// -- Usecase
			Self::Usecase(error) => error.client_status_and_error(),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Unknown Error".to_string(),
			),
		}
	}
}

impl From<usecase::error::Error> for Error {
    fn from(error: usecase::error::Error) -> Self {
        Self::Usecase(error)
    }
}

impl From<output::persistence::db::error::Error> for Error {
	fn from(error: output::persistence::db::error::Error) -> Self {
		Self::Postgres(error)
	}
}
