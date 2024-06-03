use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::{adapter::output::persistence::db::schema::{coin, network}, domain::model::{coin::Coin, coin_network::CoinNetwork, network::Network}};
use crate::port::output::coin_network_repository::CoinNetworkRepository;
use super::{adapt_db_error, coin_network};
use crate::adapter::output::persistence::db::error::{Result, Error};

#[derive(Clone, Debug)]
pub struct PostgresCoinNetworkRepository;

#[async_trait]
impl CoinNetworkRepository for PostgresCoinNetworkRepository {
    async fn get_with_coin_and_network(&self, conn: Object, coin_network_id: Uuid) -> Result<(CoinNetwork, Coin, Network)> {
        let result = conn.interact(move |conn| {
            coin_network::table
                .inner_join(coin::table.on(coin_network::coin_id.eq(coin::id)))
                .inner_join(network::table.on(coin_network::network_id.eq(network::id)))
                .filter(coin_network::id.eq(coin_network_id))
                .select((coin_network::all_columns, coin::all_columns, network::all_columns))
                .first::<(CoinNetwork, Coin, Network)>(conn)
                .map_err(adapt_db_error)
        })
        .await?
        .map_err(|e| Error::from(e));

        Ok(result.map_err(|e| Error::from(e))?)
    }

    async fn list_all(&self, conn: Object) -> Result<Vec<(CoinNetwork, Coin, Network)>>{
        let result = conn.interact(move |conn| {
            coin_network::table
                .inner_join(coin::table.on(coin_network::coin_id.eq(coin::id)))
                .inner_join(network::table.on(coin_network::network_id.eq(network::id)))
                .select((coin_network::all_columns, coin::all_columns, network::all_columns))
                .load::<(CoinNetwork, Coin, Network)>(conn)
                .map_err(adapt_db_error)
        })
        .await?
        .map_err(|e| Error::from(e));

        Ok(result.map_err(|e| Error::from(e))?)
    }

    async fn list_all_by_network_code(&self, conn: Object, network_code: String) -> Result<Vec<(CoinNetwork, Coin, Network)>> {
        tracing::info!("list_all_by_network_code: network_code={}", network_code);
        conn.interact(move |conn| {
            coin::table
                .inner_join(coin_network::table.on(coin_network::coin_id.eq(coin::id)))
                .inner_join(network::table.on(network::id.eq(coin_network::network_id)))
                .filter(network::code.ilike(network_code))
                .select((coin_network::all_columns, coin::all_columns, network::all_columns))
                .load::<(CoinNetwork, Coin, Network)>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list_all_by_ids(&self, conn: Object, coin_network_ids: Vec<Uuid>) -> Result<Vec<(CoinNetwork, Coin, Network)>> {
        let result = conn.interact(move |conn| {
            coin_network::table
                .inner_join(coin::table.on(coin_network::coin_id.eq(coin::id)))
                .inner_join(network::table.on(coin_network::network_id.eq(network::id)))
                .filter(coin_network::id.eq_any(coin_network_ids))
                .select((coin_network::all_columns, coin::all_columns, network::all_columns))
                .load::<(CoinNetwork, Coin, Network)>(conn)
                .map_err(adapt_db_error)
        })
        .await?
        .map_err(|e| Error::from(e));
    
        Ok(result.map_err(|e| Error::from(e))?)
    }

    async fn list_by_coin_id(&self, conn: Object, coin_id: Uuid) -> Result<Vec<CoinNetwork>> {
        conn.interact(move |conn| {
            coin_network::table
                .filter(coin_network::coin_id.eq(coin_id))
                .load::<CoinNetwork>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}
