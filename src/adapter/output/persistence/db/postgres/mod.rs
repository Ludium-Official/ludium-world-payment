use axum::async_trait;
use deadpool_diesel::postgres::{Manager, Object, Pool};
use diesel::prelude::*;
use uuid::Uuid;
use crate::domain::model::{Error, Result};
use crate::domain::model::user::{User, NewUser};
use crate::port::output::{UserRepository, DbManager};

use super::error::adapt_db_error;
use super::schema::tb_ldm_usr;
pub mod user_repository_impl;

#[derive(Clone)]
pub struct PostgresDbManager {
    db_pool: Pool,
}


impl PostgresDbManager {
    pub async fn new(database_url: &str) -> Result<Self> {
        let manager = Manager::new(
            database_url.to_string(),
            deadpool_diesel::Runtime::Tokio1,
        );
        let pool = Pool::builder(manager)
            .build()
            .map_err(|e| Error::from(adapt_db_error(e)))?;

        Ok(PostgresDbManager {
            db_pool: pool,
        })
    }

    pub fn from(pool: Pool) -> Self {
        PostgresDbManager { db_pool: pool }
    }
}

#[async_trait]
impl DbManager for PostgresDbManager {
    async fn get_connection(&self) -> Result<Object> {
        self.db_pool.get().await.map_err(|e| Error::from(adapt_db_error(e)))
    }
}
