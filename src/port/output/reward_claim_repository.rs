use async_trait::async_trait;
use deadpool_diesel::postgres::Object;
use uuid::Uuid;
use crate::domain::model::{reward_claim::{NewRewardClaim, ResourceType, RewardClaim, RewardClaimStatus}, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}};
use crate::adapter::output::persistence::db::error::Result;

#[async_trait]
pub trait RewardClaimRepository {
    // --- reward_claim domain
    async fn insert(&self, conn: Object, new_reward_claim: NewRewardClaim) -> Result<RewardClaim>;
    async fn get_by_resource_and_user(
        &self,
        conn: Object,
        resource_type: ResourceType,
        resource_id: Uuid,
        user_id: Uuid
    ) -> Result<RewardClaim>;

    async fn list_all_by_user(&self, conn: Object, user_id: Uuid) -> Result<Vec<(RewardClaim, RewardClaimDetail)>>;
    
    async fn update_status(&self, conn: Object, reward_claim_id: Uuid, status: RewardClaimStatus) -> Result<RewardClaim>;
    
    // --- reward_claim_detail domain
    async fn insert_detail(&self, conn: Object, new_reward_claim: NewRewardClaimDetail) -> Result<RewardClaimDetail>;
}
