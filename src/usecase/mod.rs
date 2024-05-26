use axum::http::StatusCode;

pub mod process_meta_tx;




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


// impl From<RelayError> for Error {
//     fn from(error: RelayError) -> Self {
//         Self::RelayError(error)
//     }
// }
