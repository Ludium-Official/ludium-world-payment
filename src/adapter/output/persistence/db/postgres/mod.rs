
use axum::async_trait;
use deadpool_diesel::postgres::{Manager, Object, Pool};
use deadpool_diesel::Runtime;
use crate::port::output::DbManager;
use super::error::{Result, Error, adapt_db_error};
use super::schema::{tb_ldm_usr, coin, network, coin_network, reward_claim, mission_submit, detailed_posting};
pub mod user_repository_impl;
pub mod coin_repository_impl;
pub mod network_repository_impl;
pub mod coin_network_repository_impl;
pub mod reward_claim_repository_impl;
pub mod mission_repository_impl;
pub mod detailed_posting_repository_impl;

#[derive(Clone)]
pub struct PostgresDbManager {
    db_pool: Pool
}

impl PostgresDbManager {
    pub async fn new(database_url: &str, connection_size: usize) -> Result<Self> {
        let manager = Manager::new(
            database_url.to_string(),
            Runtime::Tokio1,
        );
        
        let pool: Pool = Pool::builder(manager)
            .max_size(connection_size)
            .runtime(Runtime::Tokio1)
            .build()
            .map_err(|e| {
                tracing::error!("Failed to build pool: {:?}", e);
                Error::from(adapt_db_error(e))
            })?;

        Ok(PostgresDbManager {
            db_pool: pool,
        })
    }
}

#[async_trait]
impl DbManager for PostgresDbManager {
    async fn get_connection(&self) -> Result<Object> {
        self.db_pool.get().await.map_err(|e| Error::from(adapt_db_error(e))) 
    }
}
