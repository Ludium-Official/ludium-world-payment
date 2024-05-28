use axum::async_trait;
use deadpool_diesel::postgres::Object;
use diesel::prelude::*;
use uuid::Uuid;
use crate::{adapter::output::persistence::db::schema::reward_claim_detail, domain::model::{reward_claim::{NewRewardClaim, NewRewardClaimPayload, RewardClaim, RewardClaimStatus}, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}, Error, Result}};
use crate::port::output::reward_claim_repository::RewardClaimRepository;
use super::{adapt_db_error, reward_claim};

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
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get(&self, conn: Object, reward_claim_id: Uuid) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            reward_claim::table
                .find(reward_claim_id)
                .get_result::<RewardClaim>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn get_by_mission_and_user(&self, conn: Object, mission_id: Uuid, user_id: Uuid) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            reward_claim::table
                .filter(reward_claim::mission_id.eq(mission_id))
                .filter(reward_claim::user_id.eq(user_id))
                .first::<RewardClaim>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn list(&self, conn: Object) -> Result<Vec<RewardClaim>> {
        conn.interact(|conn| {
            reward_claim::table.load::<RewardClaim>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn update_status(
        &self, 
        conn: Object, 
        reward_claim_id: Uuid, 
        status: RewardClaimStatus, 
    ) -> Result<RewardClaim> {
        conn.interact(move |conn| {
            diesel::update(reward_claim::table.find(reward_claim_id))
                .set(reward_claim::reward_claim_status.eq(status))
                .get_result::<RewardClaim>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }

    async fn insert_detail(&self, conn: Object, new_reward_claim_detail: NewRewardClaimDetail) -> Result<RewardClaimDetail> {
        conn.interact(|conn| {
            diesel::insert_into(reward_claim_detail::table)
                .values(new_reward_claim_detail)
                .get_result::<RewardClaimDetail>(conn)
        })
        .await
        .map_err(|e| Error::from(adapt_db_error(e)))?
        .map_err(|e| Error::from(adapt_db_error(e)))
    }
}
