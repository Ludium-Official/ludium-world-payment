use axum::async_trait;
use uuid::Uuid;
use deadpool_diesel::postgres::Object;
use crate::domain::model::{coin_network::{CoinNetwork, NewCoinNetworkPayload}, Result};

#[async_trait]
pub trait CoinNetworkRepository {
    async fn insert(&self, conn: Object, new_coin_network_payload: NewCoinNetworkPayload) -> Result<CoinNetwork>;
    async fn get(&self, conn: Object, coin_network_id: Uuid) -> Result<CoinNetwork>;
    async fn list(&self, conn: Object) -> Result<Vec<CoinNetwork>>;
    async fn list_by_coin_id(&self, conn: Object, coin_id: Uuid) -> Result<Vec<CoinNetwork>>;
}
