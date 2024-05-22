use std::fmt;
use deadpool_diesel::InteractError;
use serde::Serialize;
use serde_with::serde_as;

#[serde_as]
#[derive(Debug, Serialize, Clone)]
pub enum DbError {
    ConnectionError(String),
    QueryError(String),
    PoolError(String),
    BuildError(String),
}

pub fn adapt_db_error<T: Error>(error: T) -> DbError {
    error.as_db_error()
}

impl fmt::Display for DbError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DbError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            DbError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DbError::PoolError(msg) => write!(f, "Pool error: {}", msg),
            DbError::BuildError(msg) => write!(f, "Build error: {}", msg),
        }
    }
}

pub trait Error {
    fn as_db_error(&self) -> DbError;
}

impl Error for diesel::result::Error {
    fn as_db_error(&self) -> DbError {
        match self {
            diesel::result::Error::DatabaseError(_, info) => DbError::QueryError(info.message().to_string()),
            diesel::result::Error::NotFound => DbError::QueryError("Record not found".to_string()),
            _ => DbError::QueryError("Unknown query error".to_string()), 
        }
    }
}

impl Error for deadpool_diesel::PoolError {
    fn as_db_error(&self) -> DbError {
        DbError::PoolError("Connection pool error".to_string()) 
    }
}

impl Error for InteractError {
    fn as_db_error(&self) -> DbError {
        DbError::ConnectionError("Interaction error".to_string())
    }
}

impl Error for deadpool_diesel::postgres::BuildError {
    fn as_db_error(&self) -> DbError {
        DbError::BuildError("Interaction error".to_string())
    }
}