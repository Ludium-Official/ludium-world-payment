use axum::async_trait;
use deadpool_diesel::postgres::Object;
use uuid::Uuid;
use crate::domain::model::coin::{Coin, CoinWithNetwork, NewCoinPayload};
use crate::adapter::output::persistence::db::error::Result;

#[async_trait]
pub trait CoinRepository {
    async fn get(&self, conn: Object, coin_id: Uuid) -> Result<Coin>;
    async fn list(&self, conn: Object) -> Result<Vec<Coin>>;
    async fn list_by_network_code(&self, conn: Object, network_code: String) -> Result<Vec<CoinWithNetwork>>;

    // --- test only
    async fn insert(&self, conn: Object, new_coin_payload: NewCoinPayload) -> Result<Coin>;
}