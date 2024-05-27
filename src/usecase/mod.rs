use axum::http::StatusCode;
pub mod utrait;
pub mod process_meta_tx;
pub mod reward_claim_usecase_impl;

#[derive(Debug)]
pub struct RelayError {
    // NOTE: imported StatusCode itself doesn't have a corresponding schema in the OpenAPI document
    pub(crate) status_code: StatusCode,
    pub(crate) message: String,
}

impl std::fmt::Display for RelayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.status_code, self.message)
    }
}

impl std::error::Error for RelayError {}


