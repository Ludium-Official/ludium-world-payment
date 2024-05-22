use axum::async_trait;
use deadpool_diesel::postgres::Object;
use crate::domain::model::Result;

#[async_trait]
pub trait DbManager {
    async fn get_connection(&self) -> Result<Object>;
}
