use axum::async_trait;
use uuid::Uuid;
use deadpool_diesel::postgres::Object;
use crate::domain::model::{coin::Coin, coin_network::CoinNetwork, network::Network};
use crate::adapter::output::persistence::db::error::Result;

#[async_trait]
pub trait CoinNetworkRepository {
    async fn get_with_coin_and_network(&self, conn: Object, coin_network_id: Uuid) -> Result<(CoinNetwork, Coin, Network)>;
    
    #[allow(unused)] // todo: batch
    async fn get_with_coins_and_networks(&self, conn: Object, coin_network_ids: Vec<Uuid>) -> Result<Vec<(CoinNetwork, Coin, Network)>>;
    async fn list_by_coin_id(&self, conn: Object, coin_id: Uuid) -> Result<Vec<CoinNetwork>>;
}
