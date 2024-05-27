use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::{adapter::output::persistence::db::schema::{coin, network}, domain::model::{coin::Coin, coin_network::{CoinNetwork, NewCoinNetwork, NewCoinNetworkPayload}, network::Network, Error, Result}};
use crate::port::output::coin_network_repository::CoinNetworkRepository;
use super::{adapt_db_error, coin_network};

#[derive(Clone, Debug)]
pub struct PostgresCoinNetworkRepository;

#[async_trait]
impl CoinNetworkRepository for PostgresCoinNetworkRepository {
    async fn insert(&self, conn: Object, new_coin_network_payload: NewCoinNetworkPayload) -> Result<CoinNetwork> {
        let new_coin_network = NewCoinNetwork {
            id: Uuid::new_v4(),
            coin_id: new_coin_network_payload.coin_id,
            network_id: new_coin_network_payload.network_id,
            contract_address: new_coin_network_payload.contract_address,
        };

        conn.interact(|conn| {
            diesel::insert_into(coin_network::table)
                .values(new_coin_network)
                .get_result::<CoinNetwork>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get(&self, conn: Object, coin_network_id: Uuid) -> Result<CoinNetwork> {
        conn.interact(move |conn| {
            coin_network::table
                .find(coin_network_id)
                .get_result::<CoinNetwork>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

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
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(e));

        Ok(result.map_err(|e| Error::from(e))?)
    }

    async fn get_with_coins_and_networks(&self, conn: Object, coin_network_ids: Vec<Uuid>) -> Result<Vec<(CoinNetwork, Coin, Network)>> {
        let result = conn.interact(move |conn| {
            coin_network::table
                .inner_join(coin::table.on(coin_network::coin_id.eq(coin::id)))
                .inner_join(network::table.on(coin_network::network_id.eq(network::id)))
                .filter(coin_network::id.eq_any(coin_network_ids))
                .select((coin_network::all_columns, coin::all_columns, network::all_columns))
                .load::<(CoinNetwork, Coin, Network)>(conn)
                .map_err(adapt_db_error)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(e));

        // result.map_err(adapt_db_error)
        Ok(result.map_err(|e| Error::from(e))?)
    }

    async fn list_by_coin_id(&self, conn: Object, coin_id: Uuid) -> Result<Vec<CoinNetwork>> {
        conn.interact(move |conn| {
            coin_network::table
                .filter(coin_network::coin_id.eq(coin_id))
                .load::<CoinNetwork>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list(&self, conn: Object) -> Result<Vec<CoinNetwork>> {
        conn.interact(|conn| {
            coin_network::table.load::<CoinNetwork>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}

// region: --- coin_network repository tests 

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::adapter::output::persistence::db::postgres::coin_repository_impl::PostgresCoinRepository;
    use crate::adapter::output::persistence::db::postgres::network_repository_impl::PostgresNetworkRepository;
    use crate::adapter::output::persistence::db::postgres::PostgresDbManager;
    use crate::domain::model::coin::NewCoinPayload;
    use crate::domain::model::coin_network::{NewCoinNetworkPayload, CoinNetwork};
    use crate::domain::model::network::NewNetworkPayload;
    use crate::port::output::coin_repository::CoinRepository;
    use crate::port::output::network_repository::NetworkRepository;
    use crate::port::output::DbManager;
    use uuid::Uuid;
    use serial_test::serial;

    async fn create_coin_and_network(db_manager: &PostgresDbManager) -> (Uuid, Uuid) {
        let coin_repo = PostgresCoinRepository;
        let network_repo = PostgresNetworkRepository;

        let new_coin_payload = NewCoinPayload {
            name: "Test Coin".to_string(),
            symbol: "TST".to_string(),
            coin_type: "FT".to_string(),
        };

        let new_network_payload = NewNetworkPayload {
            name: "Test Network".to_string(),
            code: "TSTNET".to_string(),
        };

        let coin = coin_repo.insert(db_manager.get_connection().await.unwrap(), new_coin_payload).await.unwrap();
        let network = network_repo.insert(db_manager.get_connection().await.unwrap(), new_network_payload).await.unwrap();

        (coin.id, network.id)
    }
    
    #[serial]
    #[tokio::test]
    async fn test_insert_and_get_coin_network() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;
        
        let (coin_id, network_id) = create_coin_and_network(&db_manager).await;

        let new_coin_network_payload = NewCoinNetworkPayload {
            coin_id,
            network_id,
            contract_address: Some("test_address".to_string()),
        };

        let inserted_coin_network = repo.insert(db_manager.get_connection().await?, new_coin_network_payload.clone()).await?;
        assert_eq!(inserted_coin_network.coin_id, new_coin_network_payload.coin_id);
        assert_eq!(inserted_coin_network.network_id, new_coin_network_payload.network_id);
        assert_eq!(inserted_coin_network.contract_address, new_coin_network_payload.contract_address);

        let fetched_coin_network = repo.get(db_manager.get_connection().await?, inserted_coin_network.id).await?;
        assert_eq!(fetched_coin_network.id, inserted_coin_network.id);
        assert_eq!(fetched_coin_network.coin_id, inserted_coin_network.coin_id);
        assert_eq!(fetched_coin_network.network_id, inserted_coin_network.network_id);

        let not_found_coin_network = repo.get(db_manager.get_connection().await?, Uuid::new_v4()).await;
        assert!(not_found_coin_network.is_err());

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_insert_coin_network_invalid_error() {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let invalid_coin_id = Uuid::new_v4(); 
        let invalid_network_id = Uuid::new_v4();  

        let new_coin_network_payload = NewCoinNetworkPayload {
            coin_id: invalid_coin_id,
            network_id: invalid_network_id,
            contract_address: Some("invalid_address".to_string()),
        };

        let result = repo.insert(db_manager.get_connection().await.unwrap(), new_coin_network_payload).await;
        assert!(result.is_err());
    }

    #[serial]
    #[tokio::test]
    async fn test_list_coin_networks() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let (coin_id, network_id) = create_coin_and_network(&db_manager).await;

        let new_coin_network_payload1 = NewCoinNetworkPayload {
            coin_id,
            network_id,
            contract_address: Some("address1".to_string()),
        };

        let new_coin_network_payload2 = NewCoinNetworkPayload {
            coin_id,
            network_id,
            contract_address: Some("address2".to_string()),
        };

        repo.insert(db_manager.get_connection().await?, new_coin_network_payload1.clone()).await?;
        repo.insert(db_manager.get_connection().await?, new_coin_network_payload2.clone()).await?;

        let coin_networks = repo.list(db_manager.get_connection().await?).await?;
        assert!(coin_networks.len() >= 2);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_coin_networks_by_coin_id() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinNetworkRepository;

        let coin_id = Uuid::new_v4();
        let network_id = Uuid::new_v4();

        let (coin_id, network_id) = create_coin_and_network(&db_manager).await;

        let new_coin_network_payload1 = NewCoinNetworkPayload {
            coin_id,
            network_id,
            contract_address: Some("address1".to_string()),
        };

        let new_coin_network_payload2 = NewCoinNetworkPayload {
            coin_id,
            network_id,
            contract_address: Some("address2".to_string()),
        };

        repo.insert(db_manager.get_connection().await?, new_coin_network_payload1.clone()).await?;
        repo.insert(db_manager.get_connection().await?, new_coin_network_payload2.clone()).await?;

        let coin_networks = repo.list_by_coin_id(db_manager.get_connection().await?, coin_id).await?;
        assert_eq!(coin_networks.len(), 2);

        Ok(())
    }
}

// endregion: --- coin_network repository tests 