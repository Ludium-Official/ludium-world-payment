use axum::async_trait;
use deadpool_diesel::postgres::Object;
use crate::adapter::output::persistence::db::error::Result;

#[async_trait]
pub trait DbManager {
    async fn get_connection(&self) -> Result<Object>;
}
