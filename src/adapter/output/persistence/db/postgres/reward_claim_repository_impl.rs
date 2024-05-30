use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::{adapter::output::persistence::db::schema::reward_claim_detail, domain::model::{reward_claim::{NewRewardClaim, NewRewardClaimPayload, RewardClaim, RewardClaimStatus}, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}}};
use crate::port::output::reward_claim_repository::RewardClaimRepository;
use super::{adapt_db_error, reward_claim};
use crate::adapter::output::persistence::db::error::{Result, Error};

#[derive(Clone, Debug)]
pub struct PostgresRewardClaimRepository;

#[async_trait]
impl RewardClaimRepository for PostgresRewardClaimRepository {
    async fn insert(&self, conn: Object, new_reward_claim_payload: NewRewardClaimPayload) -> Result<RewardClaim> {
        // TODO: validate new_reward_claim_payload
        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            mission_id: new_reward_claim_payload.mission_id,
            coin_network_id: new_reward_claim_payload.coin_network_id,
            reward_claim_status: RewardClaimStatus::Ready,
            amount: new_reward_claim_payload.amount,
            user_id: new_reward_claim_payload.user_id,
            user_address: new_reward_claim_payload.user_address,
        };

        conn.interact(|conn| {
            diesel::insert_into(reward_claim::table)
                .values(new_reward_claim)
                .get_result::<RewardClaim>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get(&self, conn: Object, reward_claim_id: Uuid) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            reward_claim::table
                .find(reward_claim_id)
                .get_result::<RewardClaim>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get_by_mission_and_user(&self, conn: Object, mission_id: Uuid, user_id: Uuid) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            reward_claim::table
                .filter(reward_claim::mission_id.eq(mission_id))
                .filter(reward_claim::user_id.eq(user_id))
                .first::<RewardClaim>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list(&self, conn: Object) -> Result<Vec<RewardClaim>> {
        conn.interact(|conn| {
            reward_claim::table.load::<RewardClaim>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn update_status(
        &self, 
        conn: Object, 
        reward_claim_id: Uuid, 
        status: RewardClaimStatus) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            diesel::update(reward_claim::table.find(reward_claim_id))
                .set((
                    reward_claim::reward_claim_status.eq(status),
                    reward_claim::updated_date.eq(diesel::dsl::now),
                ))
                .get_result::<RewardClaim>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn insert_detail(&self, conn: Object, new_reward_claim_detail: NewRewardClaimDetail) -> Result<RewardClaimDetail> {
        conn.interact(|conn| {
            diesel::insert_into(reward_claim_detail::table)
                .values(new_reward_claim_detail)
                .get_result::<RewardClaimDetail>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}


// region: --- reward claim repository tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapter::output::persistence::db::_dev_utils;
    use crate::domain::model::reward_claim::{NewRewardClaimPayload, RewardClaimStatus};
    use crate::domain::model::reward_claim_detail::NewRewardClaimDetail;
    use crate::port::output::reward_claim_repository::RewardClaimRepository;
    use crate::port::output::DbManager;
    use serial_test::serial;
    use uuid::Uuid;
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[serial]
    #[tokio::test]
    async fn test_insert_and_get_reward_claim() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim_payload = NewRewardClaimPayload {
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from_str("100").unwrap(),
            user_id: Uuid::new_v4(),
            user_address: "test_address".to_string(),
        };

        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim_payload.clone()).await?;
        assert_eq!(inserted_claim.mission_id, new_reward_claim_payload.mission_id);
        assert_eq!(inserted_claim.coin_network_id, new_reward_claim_payload.coin_network_id);
        assert_eq!(inserted_claim.amount, new_reward_claim_payload.amount);
        assert_eq!(inserted_claim.user_id, new_reward_claim_payload.user_id);
        assert_eq!(inserted_claim.user_address, new_reward_claim_payload.user_address);

        let fetched_claim = repo.get(db_manager.get_connection().await?, inserted_claim.id).await?;
        assert_eq!(fetched_claim.id, inserted_claim.id);
        assert_eq!(fetched_claim.mission_id, inserted_claim.mission_id);
        assert_eq!(fetched_claim.coin_network_id, inserted_claim.coin_network_id);
        assert_eq!(fetched_claim.amount, inserted_claim.amount);
        assert_eq!(fetched_claim.user_id, inserted_claim.user_id);
        assert_eq!(fetched_claim.user_address, inserted_claim.user_address);

        let not_found_claim = repo.get(db_manager.get_connection().await?, Uuid::new_v4()).await;
        assert!(not_found_claim.is_err());

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_get_by_mission_and_user() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim_payload = NewRewardClaimPayload {
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from_str("100").unwrap(),
            user_id: Uuid::new_v4(),
            user_address: "test_address".to_string(),
        };

        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim_payload.clone()).await?;
        let fetched_claim = repo.get_by_mission_and_user(db_manager.get_connection().await?, new_reward_claim_payload.mission_id, new_reward_claim_payload.user_id).await?;
        assert_eq!(fetched_claim.id, inserted_claim.id);
        assert_eq!(fetched_claim.mission_id, inserted_claim.mission_id);
        assert_eq!(fetched_claim.user_id, inserted_claim.user_id);

        let not_found_claim = repo.get_by_mission_and_user(db_manager.get_connection().await?, Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(not_found_claim.is_err());

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_list_reward_claims() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim_payload1 = NewRewardClaimPayload {
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from_str("100").unwrap(),
            user_id: Uuid::new_v4(),
            user_address: "test_address_1".to_string(),
        };

        let new_reward_claim_payload2 = NewRewardClaimPayload {
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from_str("200").unwrap(),
            user_id: Uuid::new_v4(),
            user_address: "test_address_2".to_string(),
        };

        repo.insert(db_manager.get_connection().await?, new_reward_claim_payload1.clone()).await?;
        repo.insert(db_manager.get_connection().await?, new_reward_claim_payload2.clone()).await?;

        let reward_claims = repo.list(db_manager.get_connection().await?).await?;
        assert!(reward_claims.len() >= 2);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_update_status() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim_payload = NewRewardClaimPayload {
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from_str("100").unwrap(),
            user_id: Uuid::new_v4(),
            user_address: "test_address".to_string(),
        };

        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim_payload.clone()).await?;
        let updated_claim = repo.update_status(db_manager.get_connection().await?, inserted_claim.id, RewardClaimStatus::TransactionApproved).await?;
        assert_eq!(updated_claim.reward_claim_status, RewardClaimStatus::TransactionApproved);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_insert_detail() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim_payload = NewRewardClaimPayload {
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: BigDecimal::from_str("100").unwrap(),
            user_id: Uuid::new_v4(),
            user_address: "test_address".to_string(),
        };

        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim_payload.clone()).await?;

        let new_reward_claim_detail = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: inserted_claim.id,
            transaction_hash: "test_hash".to_string(),
            sended_user_id: Uuid::new_v4(),
            sended_user_address: "sended_address".to_string(),
        };

        let inserted_detail = repo.insert_detail(db_manager.get_connection().await?, new_reward_claim_detail.clone()).await?;
        assert_eq!(inserted_detail.reward_claim_id, inserted_claim.id);
        assert_eq!(inserted_detail.transaction_hash, new_reward_claim_detail.transaction_hash);
        assert_eq!(inserted_detail.sended_user_id, new_reward_claim_detail.sended_user_id);
        assert_eq!(inserted_detail.sended_user_address, new_reward_claim_detail.sended_user_address);

        Ok(())
    }
}

// endregion: --- reward claim repository tests