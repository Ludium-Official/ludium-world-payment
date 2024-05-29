use async_trait::async_trait;
use bigdecimal::num_traits::sign;
use near_fetch::Client;
use near_primitives::action::delegate::SignedDelegateAction;
use near_primitives::action::Action;
use near_primitives::types::AccountId;
use near_primitives::views::{FinalExecutionOutcomeView, TxExecutionStatus};
use super::error::{Result, Error};
use crate::port::output::rpc_client::RpcClient;
use crate::config::near::KeyRotatingSignerWrapper;

#[derive(Debug, Clone)]
pub struct NearRpcManager {
    pub client: Client,
    pub signer: KeyRotatingSignerWrapper,
    pub whitelisted_contracts: Vec<String>,
    pub whitelisted_senders: Vec<String>
}

impl NearRpcManager {
    pub fn new(client: Client, 
        signer: KeyRotatingSignerWrapper,
        whitelisted_contracts: Vec<String>,
        whitelisted_senders: Vec<String>) -> Self {
        Self { client, signer, whitelisted_contracts, whitelisted_senders }
    }
}

#[async_trait]
impl RpcClient for NearRpcManager {
    async fn send_transaction(&self, signed_delegate_action: SignedDelegateAction) -> Result<FinalExecutionOutcomeView>{
        tracing::debug!(
            "Deserialized SignedDelegateAction object: {:#?}",
            signed_delegate_action
        );
    
        self.validate_signed_delegate_action(signed_delegate_action.clone()).await?;

        let receiver_id: &AccountId = &signed_delegate_action.delegate_action.sender_id;
        let actions: Vec<Action> = vec![Action::Delegate(Box::new(signed_delegate_action.clone()))];
        let execution = self.client
            .send_tx(
                self.signer.inner(),
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

    async fn validate_signed_delegate_action(&self, signed_delegate_action: SignedDelegateAction) -> Result<()> {
        tracing::debug!("whitelisted_contracts: {:?}", self.whitelisted_contracts);
        tracing::debug!("whitelisted_senders: {:?}", self.whitelisted_senders);
        if self.whitelisted_contracts.is_empty() && self.whitelisted_senders.is_empty() {
            return Ok(());
        }
    
        let sender_id = signed_delegate_action.delegate_action.sender_id;
        if !self.whitelisted_senders.contains(&sender_id.to_string()) {
            return Err(Error::NotWhitelisted { message: format!("delegated_action sender {sender_id} is not whitelisted") });
        }
    
        let delegated_action_receiver_id = signed_delegate_action.delegate_action.receiver_id;
        if self.whitelisted_contracts.contains(&delegated_action_receiver_id.to_string()) {
            return Err(Error::NotWhitelisted { message: format!("delegated_action receiver {delegated_action_receiver_id} is not whitelisted") });
        }
        Ok(())
    }
}