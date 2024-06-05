use async_trait::async_trait;
use near_primitives::{action::delegate::SignedDelegateAction, views::TxExecutionStatus};
use crate::{adapter::output::near::error::Result, domain::model::near::{TransactionResultResponse, TransferActionType}};

#[async_trait]
pub trait RpcClient: Send + Sync{
    #[allow(unused)]
    async fn relay(&self, data: Vec<u8>) -> Result<TransactionResultResponse>;
    async fn process_transfer_action(&self, transfer_action_type: TransferActionType, is_delegated: bool) -> Result<TransactionResultResponse>;
    async fn process_signed_delegate_action(
        &self,
        signed_delegate_action: &SignedDelegateAction,
        _wait_until: Option<TxExecutionStatus>,
    ) -> Result<TransactionResultResponse>;

    
}
