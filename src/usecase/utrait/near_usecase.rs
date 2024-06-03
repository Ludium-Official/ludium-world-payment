use std::sync::Arc;

use axum::async_trait;
use crate::adapter::output::near::rpc_client::NearRpcManager;
use crate::domain::model::near::TransactionResultResponse;
use crate::usecase::error::Result;
use near_primitives::views::TxExecutionStatus;
use near_primitives::action::delegate::SignedDelegateAction;

#[async_trait]
pub trait NearUsecase {
    #[allow(unused)]
    async fn relay(&self, near_rpc_manager: Arc<NearRpcManager>, data: Vec<u8>) -> Result<TransactionResultResponse>; 
    async fn process_signed_delegate_action(&self,
        near_rpc_manager: Arc<NearRpcManager>,
        signed_delegate_action: &SignedDelegateAction,
        _wait_until: Option<TxExecutionStatus>,
    ) -> Result<TransactionResultResponse>;
}
