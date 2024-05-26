mod error;

pub mod user;
pub mod near;

use std::sync::Arc;

use axum::async_trait;
use deadpool_diesel::postgres::{Manager, Object, Pool};
use diesel::PgConnection;
use tokio::sync::Mutex;

use crate::adapter::output::persistence::db::error::adapt_db_error;

pub use self::error::{Error, Result};