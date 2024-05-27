use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;
use serde_with::serde_as;

use crate::{adapter::output::persistence::db, domain::model};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	InputInvalid { field: String, message: String },
	
	// -- Mock Login Errors.
    LoginFail,

    // -- Mock Auth Errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

	// -- Model Errors.
	UserNicknameNotFound { nickname: String },
	CoinNetworkIdNotFound { id: String },
	RewardClaimDuplicate { mission_id: String, user_id: String },

	// -- Domain
	Model(model::Error),

	// -- Database
	NotFound,
    DatabaseError(db::DbError),

	// -- Payment
	PaymentFail,

	// -- Deserialization
	DeserializationError { message: String },
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

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		tracing::debug!("[into_response] - {self:?}");

		// let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// response.extensions_mut().insert(self);

		// response

		let (status, client_error) = self.client_status_and_error();
        let error_response = ErrorResponse {
            error: client_error.as_ref().to_string(),
            message: self.to_string(),
        };

        (status, Json(error_response)).into_response()
	}
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		#[allow(unreachable_patterns)]
		match self {
			Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL),

			// -- Mock Auth.
			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			}

			// -- Model.
			Self::UserNicknameNotFound { .. }
			| Self::CoinNetworkIdNotFound { .. } => {
				(StatusCode::BAD_REQUEST, ClientError::INVALID_PARAMS)
			}

			 // -- Deserialization.
			Self::DeserializationError { .. } => {
                (StatusCode::UNPROCESSABLE_ENTITY, ClientError::INVALID_PARAMS)
            }

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

impl From<model::Error> for Error {
    fn from(error: model::Error) -> Self {
        Self::Model(error)
    }
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	INVALID_PARAMS,
	SERVICE_ERROR,
}