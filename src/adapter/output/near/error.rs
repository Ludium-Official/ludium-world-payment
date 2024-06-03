use axum::http::StatusCode;
use near_jsonrpc_client::methods::tx::RpcTransactionError;
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub enum Error {
    TransactionNotExecuted { message: String },
	NotWhitelisted { message: String },
	CheckStorageDepositFailed { 
        message: String,
    },
	RpcTransactionError {
		message: String,
	},
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
			Self::TransactionNotExecuted { message } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
			Self::NotWhitelisted { message } => (
				StatusCode::FORBIDDEN,
				message.to_string(),
			),
			Self::CheckStorageDepositFailed { message } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
			Self::RpcTransactionError { message } => (
				StatusCode::INTERNAL_SERVER_ERROR,
				message.to_string(),
			),
		}
	}
}

impl From<RpcTransactionError> for Error {
	fn from(error: RpcTransactionError) -> Self {
		Self::RpcTransactionError {
			message: error.to_string()
		}
	}
}