use serde::Serialize;
use serde_with::serde_as;
use crate::adapter::output::persistence::db;
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Clone, Serialize, Debug)]
pub enum Error {
    // --- 400 
    InvalidEncodedSignedDelegateDeserialization { 
        message: String,
    },

    // --- 404
    CoinNetworkIdNotFound { 
        id: String,
    },
    RewardClaimDuplicate { 
        mission_id: String,
        user_id: String,
    },
    InvalidClaimStatusForReject, 
    InvalidClaimStatusForApprove, 

    // --- 500
    InternalServerError  { 
        message: String,
    },

    // --- External
    AdapterOutputDB(db::error::Error),
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
