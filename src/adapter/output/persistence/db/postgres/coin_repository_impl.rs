use axum::async_trait;
use deadpool_diesel::postgres::{Object, Pool};
use diesel::prelude::*;
use uuid::Uuid;
use crate::domain::model::{Error, Result};
use crate::{domain::model::coin::{Coin, CoinType, NewCoin, NewCoinPayload}, port::output::coin_repository::CoinRepository};

use super::{adapt_db_error, coin};

#[derive(Clone, Debug)]
pub struct PostgresCoinRepository;

#[async_trait]
impl CoinRepository for PostgresCoinRepository {
    async fn insert(&self, conn: Object, new_coin_payload: NewCoinPayload) -> Result<Coin> {
        let new_coin = NewCoin {
            id: Uuid::new_v4(),
            name: new_coin_payload.name,
            symbol: new_coin_payload.symbol,
            coin_type: CoinType::from(new_coin_payload.coin_type),
        };

        conn.interact(|conn| {
            diesel::insert_into(coin::table)
                .values(new_coin)
                .get_result::<Coin>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list(&self, conn: Object) -> Result<Vec<Coin>> {
        tracing::debug!("list coins");
        conn.interact(|conn| {
            coin::table.load::<Coin>(conn)
        })
        .await
        .map_err(|e| {
            tracing::error!("Error executing query: {:?}", e);
            Error::from(adapt_db_error(e))
        })?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get(&self, conn: Object, coin_id: Uuid) -> Result<Coin> {
        tracing::debug!("get coin by id: {:?}", coin_id);
        conn.interact(move |conn| {
            coin::table
                .find(coin_id)
                .get_result::<Coin>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| {
            tracing::error!("Error executing query: {:?}", e);
            Error::from(adapt_db_error(e))
    })
    }

}

// region: --- coin repository tests 
#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::domain::model::coin::{NewCoin, CoinType, Coin};
    use crate::port::output::coin_repository::CoinRepository;
    use crate::port::output::DbManager;
    use uuid::Uuid;
    use serial_test::serial;

    #[serial]
    #[tokio::test]
    async fn test_insert_and_get_coin() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinRepository;

        let new_coin_payload = NewCoinPayload {
            name: "Test Coin".to_string(),
            symbol: "TST".to_string(),
            coin_type: "FT".to_string(),
        };

        let inserted_coin = repo.insert(db_manager.get_connection().await?, new_coin_payload.clone()).await?;
        assert_eq!(inserted_coin.name, new_coin_payload.name);
        assert_eq!(inserted_coin.symbol, new_coin_payload.symbol);
        assert_eq!(inserted_coin.coin_type, CoinType::FT);

        let fetched_coin = repo.get(db_manager.get_connection().await?, inserted_coin.id).await?;
        assert_eq!(fetched_coin.id, inserted_coin.id);
        assert_eq!(fetched_coin.name, inserted_coin.name);
        assert_eq!(fetched_coin.symbol, inserted_coin.symbol);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_coins() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresCoinRepository;

        let new_coin_payload1 = NewCoinPayload {
            name: "Coin1".to_string(),
            symbol: "C1".to_string(),
            coin_type: "FT".to_string(),
        };

        let new_coin_payload2 = NewCoinPayload {
            name: "Coin2".to_string(),
            symbol: "C2".to_string(),
            coin_type: "FT".to_string(),
        };

        repo.insert(db_manager.get_connection().await?, new_coin_payload1.clone()).await?;
        repo.insert(db_manager.get_connection().await?, new_coin_payload2.clone()).await?;

        let coins = repo.list(db_manager.get_connection().await?).await?;
        assert!(coins.len() >= 2);

        Ok(())
    }
}

// endregion: --- coin repository tests