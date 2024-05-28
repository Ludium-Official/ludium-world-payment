use axum::http::StatusCode;
use serde::Serialize;
use serde_with::serde_as;


#[serde_as]
#[derive(Clone, Serialize, Debug)]
pub enum Error {
    EncodedSignedDelegateDeserializationError { // 400
        message: String,
    },
    RelayError  { // 500
        message: String,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}


