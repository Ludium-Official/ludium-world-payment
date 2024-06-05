use axum::async_trait;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use crate::domain::model::coin_network::CoinNetwork;
use crate::domain::model::near::TransactionResultResponse;
use crate::domain::model::reward_claim::{CombinedRewardClaimResponse, NewRewardClaimPayload};
use crate::usecase::error::Result;

#[async_trait]
pub trait RewardClaimUsecase {
    async fn create_reward_claim(&self, user_id: Uuid, payload: NewRewardClaimPayload) -> Result<CombinedRewardClaimResponse>;
    async fn get_me_reward_claim(&self, user_id: Uuid) -> Result<Vec<CombinedRewardClaimResponse>>;
    async fn process_native_transfer(&self, payload: NewRewardClaimPayload, amount_in_smallest_unit: BigDecimal) -> Result<TransactionResultResponse>;
    async fn process_ft_transfer(&self, coin_network: CoinNetwork, payload: NewRewardClaimPayload, amount_in_smallest_unit: BigDecimal) -> Result<TransactionResultResponse>;
}
