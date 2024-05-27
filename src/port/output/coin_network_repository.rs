use axum::async_trait;
use uuid::Uuid;
use deadpool_diesel::postgres::Object;
use crate::domain::model::{coin::Coin, coin_network::{CoinNetwork, NewCoinNetworkPayload}, network::Network, Result};

#[async_trait]
pub trait CoinNetworkRepository {
    async fn insert(&self, conn: Object, new_coin_network_payload: NewCoinNetworkPayload) -> Result<CoinNetwork>;
    async fn get(&self, conn: Object, coin_network_id: Uuid) -> Result<CoinNetwork>;
    async fn get_with_coin_and_network(&self, conn: Object, coin_network_id: Uuid) -> Result<(CoinNetwork, Coin, Network)>;
    async fn get_with_coins_and_networks(&self, conn: Object, coin_network_ids: Vec<Uuid>) -> Result<Vec<(CoinNetwork, Coin, Network)>>;
    async fn list(&self, conn: Object) -> Result<Vec<CoinNetwork>>;
    async fn list_by_coin_id(&self, conn: Object, coin_id: Uuid) -> Result<Vec<CoinNetwork>>;
}
