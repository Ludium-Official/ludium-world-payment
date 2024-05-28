use axum::async_trait;
use uuid::Uuid;
use deadpool_diesel::postgres::Object;
use crate::adapter::output::persistence::db::error::Result;
use crate::domain::model::network::{Network, NewNetworkPayload};

#[async_trait]
pub trait NetworkRepository {
    //! test only
    async fn insert(&self, conn: Object, new_network_payload: NewNetworkPayload) -> Result<Network>;
    async fn get(&self, conn: Object, id: Uuid) -> Result<Network>;
    async fn list(&self, conn: Object) -> Result<Vec<Network>>;
}
