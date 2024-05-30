use async_trait::async_trait;
use deadpool_diesel::postgres::Object;
use uuid::Uuid;
use crate::domain::model::{reward_claim::{NewRewardClaimPayload, RewardClaim, RewardClaimStatus}, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}};
use crate::adapter::output::persistence::db::error::Result;

#[async_trait]
pub trait RewardClaimRepository {
    // --- reward_claim domain
    async fn insert(&self, conn: Object, new_reward_claim_payload: NewRewardClaimPayload) -> Result<RewardClaim>;
    async fn get(&self, conn: Object, reward_claim_id: Uuid) -> Result<RewardClaim>;
    async fn get_by_mission_and_user(&self, conn: Object, mission_id: Uuid, user_id: Uuid) -> Result<RewardClaim>;
    async fn list(&self, conn: Object) -> Result<Vec<RewardClaim>>;
    async fn update_status(&self, conn: Object, reward_claim_id: Uuid, status: RewardClaimStatus) -> Result<RewardClaim>;
    
    // --- reward_claim_detail domain
    async fn insert_detail(&self, conn: Object, new_reward_claim_payload: NewRewardClaimDetail) -> Result<RewardClaimDetail>;

}
