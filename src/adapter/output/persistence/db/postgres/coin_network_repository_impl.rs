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
        .await?;

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
        .await?;

        Ok(result.map_err(|e| Error::from(e))?)
    }

    async fn list_all_by_network_code(&self, conn: Object, network_code: String) -> Result<Vec<(CoinNetwork, Coin, Network)>> {
        conn.interact(move |conn| {
            coin::table
                .inner_join(coin_network::table.on(coin_network::coin_id.eq(coin::id)))
                .inner_join(network::table.on(network::id.eq(coin_network::network_id)))
                .filter(network::code.ilike(network_code))
                .select((coin_network::all_columns, coin::all_columns, network::all_columns))
                .load::<(CoinNetwork, Coin, Network)>(conn)
                .map_err(adapt_db_error)
        })
        .await?
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
        .await?;
    
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


// region: --- coin network repository tests 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{adapter::output::persistence::db::_dev_utils, port::output::DbManager};
    use uuid::Uuid;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_get_with_coin_and_network() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let coin_network_id = Uuid::parse_str("22222222-0000-0000-0000-000000000001").unwrap();
        
        let result = repo.get_with_coin_and_network(db_manager.get_connection().await?, coin_network_id).await?;
        assert_eq!(result.0.id, coin_network_id);
        assert_eq!(result.1.name, "USD Tether");
        assert_eq!(result.2.name, "NEAR Protocol");

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let result = repo.list_all(db_manager.get_connection().await?).await?;
        assert!(result.len() >= 3);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all_by_network_code() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let result = repo.list_all_by_network_code(db_manager.get_connection().await?, "NEAR".to_string()).await?;
        assert!(result.len() >= 3);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_all_by_ids() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let coin_network_ids = vec![
            Uuid::parse_str("22222222-0000-0000-0000-000000000001").unwrap(),
            Uuid::parse_str("22222222-9c58-47f8-9a0f-2d0c8d3f807f").unwrap(),
        ];

        let result = repo.list_all_by_ids(db_manager.get_connection().await?, coin_network_ids).await?;
        assert_eq!(result.len(), 2);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_by_coin_id() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let coin_id = Uuid::parse_str("11111111-0000-0000-0000-000000000001").unwrap();

        let result = repo.list_by_coin_id(db_manager.get_connection().await?, coin_id).await?;
        assert!(result.len() >= 1);

        Ok(())
    }
}

// endregion: --- coin network repository tests 