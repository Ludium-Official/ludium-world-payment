use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::{adapter::output::persistence::db::schema::reward_claim_detail, domain::model::{reward_claim::{NewRewardClaim, RewardClaim}, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}}};
use crate::port::output::reward_claim_repository::RewardClaimRepository;
use super::{adapt_db_error, reward_claim};
use crate::adapter::output::persistence::db::error::{Result, Error};

#[derive(Clone, Debug)]
pub struct PostgresRewardClaimRepository;

#[async_trait]
impl RewardClaimRepository for PostgresRewardClaimRepository {
    async fn insert(&self, conn: Object, new_reward_claim: NewRewardClaim) -> Result<RewardClaim> {
        conn.interact(|conn| {
            diesel::insert_into(reward_claim::table)
                .values(new_reward_claim)
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

    async fn insert_detail(&self, conn: Object, new_reward_claim_detail: NewRewardClaimDetail) -> Result<RewardClaimDetail> {
        conn.interact(|conn| {
            diesel::insert_into(reward_claim_detail::table)
                .values(new_reward_claim_detail)
                .get_result::<RewardClaimDetail>(conn)
        })
        .await?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list_all_by_user(&self, conn: Object, user_id: Uuid) -> Result<Vec<(RewardClaim, RewardClaimDetail)>> {
        conn.interact(move |conn| {
            reward_claim::table
                .filter(reward_claim::user_id.eq(user_id))
                .inner_join(reward_claim_detail::table)
                .load::<(RewardClaim, RewardClaimDetail)>(conn)
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
    use crate::domain::model::reward_claim::RewardClaimStatus;
    use crate::domain::model::reward_claim_detail::NewRewardClaimDetail;
    use crate::port::output::reward_claim_repository::RewardClaimRepository;
    use crate::port::output::DbManager;
    use serial_test::serial;
    use uuid::Uuid;

    #[serial]
    #[tokio::test]
    async fn test_get_by_mission_and_user() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: 20000,
            user_id: Uuid::new_v4(),
            user_address: "test_address_1".to_string(),
            reward_claim_status: RewardClaimStatus::TransactionApproved,
        };

        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim.clone()).await?;
        let fetched_claim = repo.get_by_mission_and_user(db_manager.get_connection().await?, new_reward_claim.mission_id, new_reward_claim.user_id).await?;
        assert_eq!(fetched_claim.id, inserted_claim.id);
        assert_eq!(fetched_claim.mission_id, inserted_claim.mission_id);
        assert_eq!(fetched_claim.user_id, inserted_claim.user_id);

        let not_found_claim = repo.get_by_mission_and_user(db_manager.get_connection().await?, Uuid::new_v4(), Uuid::new_v4()).await;
        assert!(not_found_claim.is_err());

        Ok(())
    }


    #[serial]
    #[tokio::test]
    async fn test_insert_detail() -> Result<()> {
        let db_manager = _dev_utils::init_test().await;
        let repo = PostgresRewardClaimRepository;

        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            mission_id: Uuid::new_v4(),
            coin_network_id: Uuid::new_v4(),
            amount: 10000,
            user_id: Uuid::new_v4(),
            user_address: "test_address".to_string(),
            reward_claim_status: RewardClaimStatus::TransactionApproved,
        };
        
        let inserted_claim = repo.insert(db_manager.get_connection().await?, new_reward_claim.clone()).await?;

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