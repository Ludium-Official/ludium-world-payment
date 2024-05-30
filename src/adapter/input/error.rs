use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use serde_with::serde_as;

use crate::{adapter::output::{self, near, persistence::db}, usecase};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Clone, Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- Mock Login Errors.
    LoginFail,

    // -- Mock Auth Errors.
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailCtxNotInRequestExt,

	// -- Mock Errors
	UserNicknameNotFound { nickname: String },

	// -- Request Params
	UUIDParsingError { message: String },
	
	// -- Output
	Postgres(db::error::Error),
	Near(near::error::Error),

	// -- Usecase 
	Usecase(usecase::error::Error),
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

		let (status, client_error_message) = self.client_status_and_error();
        (status, client_error_message.to_string()).into_response()
	}
}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, String) {
		#[allow(unreachable_patterns)]
		match self {
			// -- Mock 
			Self::LoginFail => (StatusCode::FORBIDDEN, "Login Failed".to_string()),
			Self::AuthFailNoAuthTokenCookie
			| Self::AuthFailTokenWrongFormat
			| Self::AuthFailCtxNotInRequestExt => {
				(StatusCode::FORBIDDEN, "No Auth".to_string())
			}
			Self::UserNicknameNotFound { .. } => {
				(StatusCode::BAD_REQUEST, "User Nickname Not Found,".to_string())
			}

			// -- Request Params
			Self::UUIDParsingError { message } => (
				StatusCode::BAD_REQUEST,
				message.clone(),
			),

			// -- Output
			Self::Postgres { .. } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Postgres Error".to_string(),
			),

			Self::Near(near::error::Error::TransactionNotExecuted { message }) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
			Self::Near(near::error::Error::NotWhitelisted { message }) => (
				StatusCode::FORBIDDEN,
				message.to_string(),
			),

			// -- Usecase
			Self::Usecase(usecase::error::Error::InvalidClaimStatusForReject) => (
				StatusCode::BAD_REQUEST,
				"Invalid Claim Status For Reject".to_string(),
			),
			Self::Usecase(usecase::error::Error::InvalidClaimStatusForApprove) => (
				StatusCode::BAD_REQUEST,
				"Invalid Claim Status For Approve".to_string(),
			),
			Self::Usecase(usecase::error::Error::CoinNetworkIdNotFound { id }) => (
				StatusCode::NOT_FOUND,
				format!("Coin Network Id Not Found: {}", id)
			),
			Self::Usecase(usecase::error::Error::RewardClaimDuplicate { mission_id, user_id }) => (
				StatusCode::BAD_REQUEST,
				format!("Reward Claim Duplicate: Mission Id: {}, User Id: {}", mission_id, user_id)
			),
			Self::Usecase(usecase::error::Error::InvalidEncodedSignedDelegateDeserialization { message }) => (
				StatusCode::BAD_REQUEST,
				message.to_string(),
			),
			Self::Usecase(usecase::error::Error::InternalServerError { message }) => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),

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
