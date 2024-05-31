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

    // --- 404
    CoinTypeNotSupported{
        coin_type: String,
    },
    CoinNetworkIdNotFound { 
        id: String,
    },
    RewardClaimDuplicate { 
        mission_id: String,
        user_id: String,
    },
    InvalidClaimStatusForReject, 
    InvalidClaimStatusForApprove, 
    TransactionActionFailed { 
        message: String,
    },
    TransactionUnknownAction { 
        message: String,
    },
    InvalidAmountConversion,

    // --- 500
    InternalServerError  { 
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