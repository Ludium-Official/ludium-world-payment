use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::domain::model::{network::{Network, NewNetwork, NewNetworkPayload}, Error, Result};
use crate::port::output::network_repository::NetworkRepository;
use super::{adapt_db_error, network};

#[derive(Clone, Debug)]
pub struct PostgresNetworkRepository;

#[async_trait]
impl NetworkRepository for PostgresNetworkRepository {
    async fn insert(&self, conn: Object, new_network_payload: NewNetworkPayload) -> Result<Network> {
        let new_network = NewNetwork {
            id: Uuid::new_v4(),
            name: new_network_payload.name,
            code: new_network_payload.code,
        };

        conn.interact(|conn| {
            diesel::insert_into(network::table)
                .values(new_network)
                .get_result::<Network>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get(&self, conn: Object, id: Uuid) -> Result<Network> {
        conn.interact(move |conn| {
            network::table.find(id).get_result::<Network>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list(&self, conn: Object) -> Result<Vec<Network>> {
        conn.interact(|conn| {
            network::table.load::<Network>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}

// region: --- network repository tests 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::domain::model::network::{NewNetwork, Network};
    use crate::port::output::network_repository::NetworkRepository;
    use crate::port::output::DbManager;
    use uuid::Uuid;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_insert_and_get_network() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresNetworkRepository;

        let new_network_payload = NewNetworkPayload {
            name: "Test Network".to_string(),
            code: "TSTNET".to_string(),
        };

        let inserted_network = repo.insert(db_manager.get_connection().await?, new_network_payload.clone()).await?;
        assert_eq!(inserted_network.name, new_network_payload.name);
        assert_eq!(inserted_network.code, new_network_payload.code);

        let fetched_network = repo.get(db_manager.get_connection().await?, inserted_network.id).await?;
        assert_eq!(fetched_network.id, inserted_network.id);
        assert_eq!(fetched_network.name, inserted_network.name);
        assert_eq!(fetched_network.code, inserted_network.code);

        let not_found_network = repo.get(db_manager.get_connection().await?, Uuid::new_v4()).await;
        assert!(not_found_network.is_err());

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_networks() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresNetworkRepository;

        let new_network_payload1 = NewNetworkPayload {
            name: "Network1".to_string(),
            code: "NET1".to_string(),
        };

        let new_network_payload2 = NewNetworkPayload {
            name: "Network2".to_string(),
            code: "NET2".to_string(),
        };

        repo.insert(db_manager.get_connection().await?, new_network_payload1.clone()).await?;
        repo.insert(db_manager.get_connection().await?, new_network_payload2.clone()).await?;

        let networks = repo.list(db_manager.get_connection().await?).await?;
        assert!(networks.len() >= 2);

        Ok(())
    }
}
// endregion: --- network repository tests 
