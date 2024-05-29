use async_trait::async_trait;
use near_fetch::Client;
use near_primitives::action::delegate::SignedDelegateAction;
use near_primitives::action::Action;
use near_primitives::types::AccountId;
use near_primitives::views::{FinalExecutionOutcomeView, TxExecutionStatus};
use super::error::{Result, Error};
use crate::port::output::rpc_client::RpcClient;
use crate::config::near::ROTATING_SIGNER;

#[derive(Debug, Clone)]
pub struct NearRpcManager {
    pub client: Client,
}

impl NearRpcManager {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl RpcClient for NearRpcManager {
    async fn send_transaction(&self, signed_delegate_action: SignedDelegateAction) -> Result<FinalExecutionOutcomeView>{
        tracing::debug!(
            "Deserialized SignedDelegateAction object: {:#?}",
            signed_delegate_action
        );
    
        let receiver_id: &AccountId = &signed_delegate_action.delegate_action.sender_id;
        let actions: Vec<Action> = vec![Action::Delegate(Box::new(signed_delegate_action.clone()))];
    
        let execution = self.client
            .send_tx(
                &*ROTATING_SIGNER,
                receiver_id,
                actions,
                Some(TxExecutionStatus::ExecutedOptimistic),
            )
            .await
            .map_err(|err| {
                let err_msg = format!("Error signing transaction: {err:?}");
                tracing::error!("{err_msg}");
                Error::TransactionNotExecuted {
                    message: err_msg,
                }
            })?;
    
        Ok(execution)
    }
}
