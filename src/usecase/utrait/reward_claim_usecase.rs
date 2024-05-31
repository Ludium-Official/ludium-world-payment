use axum::async_trait;
use uuid::Uuid;
use crate::domain::model::reward_claim::{CombinedRewardClaimResponse, NewRewardClaimPayload};
use crate::usecase::error::Result;

#[async_trait]
pub trait RewardClaimUsecase {
    async fn create_reward_claim(&self, user_id: Uuid, payload: NewRewardClaimPayload) -> Result<CombinedRewardClaimResponse>;
}
