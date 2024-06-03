use axum::http::StatusCode;
use serde::Serialize;
use serde_with::serde_as;
use crate::adapter::output::{near, persistence::db};
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Clone, Serialize, Debug)]
pub enum Error {
    // --- 400 
    InvalidEncodedSignedDelegateDeserialization { 
        message: String,
    },
    RewardClaimDuplicate { 
        mission_id: String,
        user_id: String,
    },
    TranscationActionVerifyFailed,
    InvalidClaimStatusForReject, 
    InvalidClaimStatusForApprove, 
    InvalidAmountConversion,

    // --- 404
    CoinTypeNotSupported{
        coin_type: String,
    },
    CoinNetworkIdNotFound { 
        id: String,
    },

    // --- 500
    InternalServerError  { 
        message: String,  
    },
    TranscationTimeoutFailed {
        message: String,
    },
    TransactionActionFailed { 
        message: String,
    },

    // --- External
    AdapterOutputDB(db::error::Error),
    AdapterOutptuNear(near::error::Error),
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
            Self::CoinTypeNotSupported { coin_type } => (
                StatusCode::NOT_FOUND,
                format!("Coin Type Not Supported: {}", coin_type),
            ),
            Self::CoinNetworkIdNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Coin Network Id Not Found: {}", id),
            ),
            Self::RewardClaimDuplicate { mission_id, user_id } => (
                StatusCode::BAD_REQUEST,
                format!("Reward Claim Duplicate: Mission Id: {}, User Id: {}", mission_id, user_id),
            ),
            Self::TranscationActionVerifyFailed => (
                StatusCode::BAD_REQUEST,
                "Transaction Action Verify Failed".to_string(),
            ),
            Self::InvalidClaimStatusForReject => (
                StatusCode::BAD_REQUEST,
                "Invalid Claim Status For Reject".to_string(),
            ),
            Self::InvalidClaimStatusForApprove => (
                StatusCode::BAD_REQUEST,
                "Invalid Claim Status For Approve".to_string(),
            ),
            Self::InvalidAmountConversion => (
                StatusCode::BAD_REQUEST,
                "Invalid Amount Conversion".to_string(),
            ),
            Self::InternalServerError { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
            Self::TranscationTimeoutFailed { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Transaction Timeout Failed".to_string(),
            ),
            Self::TransactionActionFailed { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Transaction Action Failed".to_string(),
            ),

            Self::AdapterOutputDB(error) => error.client_status_and_error(),
            Self::AdapterOutptuNear(error) => error.client_status_and_error(),

			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				"Unknown Error".to_string(),
			),
		}
	}
}


impl From<db::Error> for Error {
    fn from(error: db::Error) -> Self {
        Self::AdapterOutputDB(error)
    }
}

impl From<near::error::Error> for Error {
    fn from(error: near::error::Error) -> Self {
        Self::AdapterOutptuNear(error)
    }
}