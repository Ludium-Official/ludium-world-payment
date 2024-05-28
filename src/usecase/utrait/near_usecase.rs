use std::sync::Arc;

use axum::async_trait;
use uuid::Uuid;
use crate::domain::model::near::TransactionResultResponse;
use crate::domain::model::reward_claim::{CombinedRewardClaimResponse, NewRewardClaimPayload, RewardClaimApprovePayload, RewardClaimApproveResponse, RewardClaimResponse};
use crate::port::output::rpc_client::RpcClient;

#[async_trait]
pub trait NearUsecase {
    async fn relay(
        near_rpc_manager: &Arc<dyn RpcClient>,
        data: Vec<u8>,
    ) -> String ;
}
