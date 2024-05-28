use async_trait::async_trait;
use near_primitives::{action::delegate::SignedDelegateAction, views::FinalExecutionOutcomeView};
// use crate::domain::model::Result;
use crate::adapter::input::error::Error;

#[async_trait]
pub trait RpcClient: Send + Sync{
    async fn send_transaction(&self, signed_delegate_action: SignedDelegateAction) -> Result<FinalExecutionOutcomeView, Error>;
}
