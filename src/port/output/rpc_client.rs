use async_trait::async_trait;
use near_primitives::{action::delegate::SignedDelegateAction, views::FinalExecutionOutcomeView};
use crate::adapter::output::near::error::Result;

#[async_trait]
pub trait RpcClient: Send + Sync{
    async fn send_transaction(&self, signed_delegate_action: SignedDelegateAction) -> Result<FinalExecutionOutcomeView>;
    async fn validate_signed_delegate_action(&self, signed_delegate_action: SignedDelegateAction) -> Result<()>;
}
