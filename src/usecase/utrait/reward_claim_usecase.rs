use axum::async_trait;
use crate::adapter::input::error::{Result, Error};
use crate::domain::model::reward_claim::{NewRewardClaimPayload, RewardClaimResponse};

#[async_trait]
pub trait RewardClaimUsecase {
    async fn create_reward_claim(&self, payload: NewRewardClaimPayload) -> Result<RewardClaimResponse>;
    async fn create_multiple_reward_claims(&self, payloads: Vec<NewRewardClaimPayload>) -> Result<Vec<RewardClaimResponse>>;
}
