use std::str::FromStr;
use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use near_fetch::signer::KeyRotatingSigner;
use near_fetch::Client;
use near_primitives::action::delegate::{DelegateAction, NonDelegateAction, SignedDelegateAction};
use near_primitives::action::{Action, FunctionCallAction, TransferAction};
use near_primitives::signable_message::{SignableMessage, SignableMessageType};
use near_primitives::types::{AccountId, Balance, BlockHeight};
use near_fetch::signer::ExposeAccountId;
use near_primitives::views::{FinalExecutionOutcomeView, TxExecutionStatus};
use serde_json::json;
use super::error::{Result, Error};
use crate::domain::model::near::TransferActionType;
use crate::port::output::rpc_client::RpcClient;
use crate::config::near::KeyRotatingSignerWrapper;

#[derive(Debug, Clone)]
pub struct NearRpcManager {
    pub client: Client,
    pub signer: KeyRotatingSignerWrapper,
    pub whitelisted_contracts: Vec<String>,
    pub whitelisted_senders: Vec<String>,
    pub nonce_mutex: Arc<Mutex<u64>>,
}

impl NearRpcManager {
    pub fn new(client: Client, 
        signer: KeyRotatingSignerWrapper,
        whitelisted_contracts: Vec<String>,
        whitelisted_senders: Vec<String>,
    ) -> Self {
        Self { client, signer, whitelisted_contracts, whitelisted_senders, nonce_mutex: Arc::new(Mutex::new(0)) }
    }

    pub fn signer(&self) -> &KeyRotatingSigner{
        self.signer.inner()
    }

    async fn get_next_nonce(&self) -> Result<u64> {
        let mut nonce = self.nonce_mutex.lock().unwrap();
        *nonce += 20;
        Ok(*nonce)
    }

    // async fn get_next_nonce(&self, account_id: &AccountId, public_key: &str) -> Result<u64> {
    //     let (nonce, _block_hash, _) = self
    //         .client
    //         .fetch_nonce(account_id, public_key)
    //         .await
    //         .map_err(|e| Error::InternalServerError { message: e.to_string() })?;
    //     Ok(nonce + 1)
    // }
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
                self.signer(),
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
        if self.whitelisted_contracts.is_empty() && self.whitelisted_senders.is_empty() {
            return Ok(());
        }
    
        let sender_id = signed_delegate_action.delegate_action.sender_id;
        if !self.whitelisted_senders.is_empty() && !self.whitelisted_senders.contains(&sender_id.to_string()) {
            return Err(Error::NotWhitelisted { message: format!("delegated_action sender {sender_id} is not whitelisted") });
        }
    
        let delegated_action_receiver_id = signed_delegate_action.delegate_action.receiver_id;
        if !self.whitelisted_contracts.is_empty() && !self.whitelisted_contracts.contains(&delegated_action_receiver_id.to_string()) {
            return Err(Error::NotWhitelisted { message: format!("delegated_action receiver {delegated_action_receiver_id} is not whitelisted") });
        }
        Ok(())
    }

    async fn create_transfer_signed_delegate_action(
        &self,
        transfer_action_type: TransferActionType,
    ) -> Result<SignedDelegateAction> {
        match transfer_action_type {
            TransferActionType::Native { user_address, amount_in_smallest_unit } => {
                let receiver_id = AccountId::from_str(&user_address).unwrap();
                let actions = vec![Action::Transfer(TransferAction {
                    deposit: amount_in_smallest_unit as Balance,
                })];

                self.create_signed_delegate_action(receiver_id, actions).await
            }
            TransferActionType::FtTransfer { ft_contract_id, user_address, amount_in_smallest_unit } => {
                let receiver_id = AccountId::from_str(&user_address).unwrap();
                let args: serde_json::Value = json!({
                    "receiver_id": receiver_id.to_string(),
                    "amount": amount_in_smallest_unit.to_string()
                });
                let function_call_action = FunctionCallAction {
                    method_name: "ft_transfer".to_string(),
                    args: args.to_string().into_bytes(),
                    gas: 100_000_000_000_000, // 100 Tgas
                    deposit: 1, // 1 yoctoNEAR for the function call
                };

                let actions = vec![Action::FunctionCall(Box::new(function_call_action))];
                self.create_signed_delegate_action(ft_contract_id, actions).await
            }
        }
    }

    async fn create_signed_delegate_action(
        &self,
        receiver_id: AccountId,
        actions: Vec<Action>,
    ) -> Result<SignedDelegateAction> {
        let signer = self.signer();
        let relayer_account_id: AccountId = signer.account_id().clone();
        let public_key = signer.public_key();
        let (nonce, _block_hash, _) = self
            .client
            .fetch_nonce(&relayer_account_id, &public_key.clone())
            .await
            .unwrap();

        let add_nonce = self.get_next_nonce().await?;
        let last_nonce = nonce + add_nonce;
        tracing::debug!("nonce: {}, add_nonce: {}, last_nonce: {}", nonce, add_nonce, last_nonce);

        let delegate_action = DelegateAction {
            sender_id: relayer_account_id.clone(),
            receiver_id: receiver_id,
            actions: actions
                .into_iter()
                .map(|a| NonDelegateAction::try_from(a).unwrap())
                .collect(),
            nonce: last_nonce,
            max_block_height: 2000000000 as BlockHeight,
            public_key: public_key.clone(),
        };

        let signable = SignableMessage::new(&delegate_action, SignableMessageType::DelegateAction);
        Ok(SignedDelegateAction {
            signature: signable.sign(signer),
            delegate_action,
        })
    }
    
    
    async fn send_storage_deposit(
        &self,
        contract_id: AccountId,
        receiver_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView>{
        let signer = self.signer();
    
        let storage_deposit_args = json!({
            "account_id": receiver_id.to_string(),
            "registration_only": true
        });
    
        let storage_deposit_action = FunctionCallAction {
            method_name: "storage_deposit".to_string(),
            args: storage_deposit_args.to_string().into_bytes(),
            gas: 100_000_000_000_000, // 100 Tgas
            deposit: 125 * 10u128.pow(19), // 0.00125near 
        };
    
        let actions = vec![Action::FunctionCall(Box::new(storage_deposit_action))];
    
        let execution = self.client
            .send_tx(
                signer,
                &contract_id,
                actions,
                Some(TxExecutionStatus::ExecutedOptimistic),
            )
            .await
            .map_err(|err| {
                let err_msg = format!("Error storage_deposit transaction: {err:?}");
                tracing::error!("{err_msg}");
                Error::TransactionNotExecuted { message: err_msg }
            })?;
    
        Ok(execution)
    }
}