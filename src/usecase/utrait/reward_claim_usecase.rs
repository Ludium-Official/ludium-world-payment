use axum::async_trait;
use uuid::Uuid;
use crate::domain::model::reward_claim::{CombinedRewardClaimResponse, NewRewardClaimPayload, RewardClaimApprovePayload, RewardClaimApproveResponse, RewardClaimResponse};
use crate::usecase::error::Result;

#[async_trait]
pub trait RewardClaimUsecase {
    async fn create_reward_claim(&self, payload: NewRewardClaimPayload) -> Result<CombinedRewardClaimResponse>;
    async fn create_multiple_reward_claims(&self, payloads: Vec<NewRewardClaimPayload>) -> Result<Vec<CombinedRewardClaimResponse>>;
    async fn reject_reward_claim(&self, claim_id: Uuid) -> Result<RewardClaimResponse>;
    async fn approve_reward_claim(&self, claim_id: Uuid, reward_claim_approve_payload: RewardClaimApprovePayload) -> Result<RewardClaimApproveResponse>;

}
