use axum::http::StatusCode;
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub enum Error {
	// --- 400
	InvalidEncodedSignedDelegateDeserialization { 
        message: String,
    },

	// --- 403 
	NotWhitelisted { message: String },

	// --- 500
	CustomInvalidNonce,
	CustomInvalidSignature,
	CustomInvalidTxError{
		message: String,
	},
	TransactionNotExecuted { 
		message: String 
	},
	CheckStorageDepositFailed { 
        message: String,
    },
	InternalServerError {
		message: String,
	}
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, String) {
		#[allow(unreachable_patterns)]
		match self {
			Self::InvalidEncodedSignedDelegateDeserialization { message } => (
                StatusCode::BAD_REQUEST,
                message.to_string()
            ),
			Self::NotWhitelisted { message } => (
				StatusCode::FORBIDDEN,
				message.to_string(),
			),
			Self::TransactionNotExecuted { message } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
			Self::CheckStorageDepositFailed { message } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
			Self::CustomInvalidNonce => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Transaction is not signed with the given public key".to_string(),
			),
			Self::CustomInvalidSignature => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Invalid signature".to_string(),
			),
			Self::CustomInvalidTxError { message } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
			Self::InternalServerError { message } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
		}
	}
}

