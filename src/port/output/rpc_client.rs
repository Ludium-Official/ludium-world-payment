use async_trait::async_trait;
use near_primitives::{action::{delegate::SignedDelegateAction, Action}, types::AccountId, views::FinalExecutionOutcomeView};
use crate::{adapter::output::near::error::Result, domain::model::near::TransferActionType};

#[async_trait]
pub trait RpcClient: Send + Sync{
    async fn send_transaction(&self, signed_delegate_action: SignedDelegateAction) -> Result<FinalExecutionOutcomeView>;
    async fn validate_signed_delegate_action(&self, signed_delegate_action: SignedDelegateAction) -> Result<()>;
    async fn send_storage_deposit(&self, contract_id: AccountId, receiver_id: AccountId) -> Result<FinalExecutionOutcomeView>;
    async fn create_transfer_signed_delegate_action(&self, transfer_action_type: TransferActionType) -> Result<SignedDelegateAction>;
    async fn create_signed_delegate_action(&self, receiver_id: AccountId, actions: Vec<Action>) -> Result<SignedDelegateAction> ;
}
