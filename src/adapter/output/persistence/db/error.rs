use deadpool_diesel::InteractError;
use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub enum Error {
    ConnectionError(String),
    QueryError(String),
    PoolError(String),
    BuildError(String),
}

pub fn adapt_db_error<T: PgError>(error: T) -> Error {
    error.as_db_error()
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Error::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            Error::QueryError(msg) => write!(f, "Query error: {}", msg),
            Error::PoolError(msg) => write!(f, "Pool error: {}", msg),
            Error::BuildError(msg) => write!(f, "Build error: {}", msg),
        }
    }
}

pub trait PgError {
    fn as_db_error(&self) -> Error;
}

impl PgError for diesel::result::Error {
    fn as_db_error(&self) -> Error {
        match self {
            diesel::result::Error::DatabaseError(_, info) => Error::QueryError(info.message().to_string()),
            diesel::result::Error::NotFound => Error::QueryError("Record not found".to_string()),
            _ => Error::QueryError("Unknown query error".to_string()), 
        }
    }
}

impl PgError for deadpool_diesel::PoolError {
    fn as_db_error(&self) -> Error {
        Error::PoolError("get db connection pool error".to_string()) 
    }
}

impl PgError for InteractError {
    fn as_db_error(&self) -> Error {
        Error::ConnectionError("Interaction error".to_string())
    }
}

impl PgError for deadpool_diesel::postgres::BuildError {
    fn as_db_error(&self) -> Error {
        Error::BuildError("Interaction error".to_string())
    }
}

// --- region: From trait implementations
impl From<deadpool_diesel::PoolError> for Error {
    fn from(error: deadpool_diesel::PoolError) -> Self {
        error.as_db_error()
    }
}

impl From<InteractError> for Error {
    fn from(error: InteractError) -> Self {
        error.as_db_error()
    }
}

impl From<deadpool_diesel::postgres::BuildError> for Error {
    fn from(error: deadpool_diesel::postgres::BuildError) -> Self {
        error.as_db_error()
    }
}
// --- endregion: From trait implementations