use std::str::FromStr;
use std::sync::Arc;
use bigdecimal::ToPrimitive;
use async_trait::async_trait;
use near_fetch::signer::KeyRotatingSigner;
use near_fetch::Client;
use near_fetch::Error as NearFetchError;
use near_jsonrpc_client::errors::JsonRpcError;
use near_jsonrpc_client::errors::JsonRpcServerError;
use near_fetch::Error::RpcTransactionError as ParentRpcTransactionError;
use near_jsonrpc_primitives::types::transactions::RpcTransactionError::InvalidTransaction;
use near_primitives::errors::ActionError as TxActionError;
use near_primitives::errors::ActionErrorKind;
use near_primitives::errors::InvalidTxError;
use near_primitives::action::delegate::{DelegateAction, NonDelegateAction, SignedDelegateAction};
use near_primitives::action::{Action, FunctionCallAction, TransferAction};
use near_primitives::borsh::BorshDeserialize;
use near_primitives::errors::TxExecutionError;
use near_primitives::signable_message::{SignableMessage, SignableMessageType};
use near_primitives::types::{AccountId, BlockHeight};
use near_primitives::views::ExecutionStatusView;
use near_primitives::views::{FinalExecutionOutcomeView, TxExecutionStatus};
use serde_json::json;
use tokio::sync::Mutex;
use near_fetch::signer::ExposeAccountId;
use super::error::{Result, Error};
use crate::domain::model::near::TransactionResultResponse;
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

    fn signer(&self) -> Arc<KeyRotatingSigner> {
        self.signer.inner()
    }

    fn handle_transaction_error(&self, err: NearFetchError) -> Error {
        match err {
            ParentRpcTransactionError(JsonRpcError::ServerError(JsonRpcServerError::HandlerError(InvalidTransaction { context }))) => {
                match context {
                    InvalidTxError::InvalidNonce { .. } => {
                        tracing::warn!("Invalid nonce: {context:?}");
                        Error::CustomInvalidNonce
                    },
                    InvalidTxError::InvalidSignature { .. } => {
                        tracing::warn!("Invalid signature: {context:?}");
                        Error::CustomInvalidSignature
                    },
                    _ => Error::CustomInvalidTxError { message: context.to_string() }
                }
            },
            _ => {
                let err_msg = format!("Error transaction: {err:?}");
                tracing::error!("{err_msg}");
                Error::TransactionNotExecuted { message: err_msg }
            }
        }
    }

    async fn execute_actions(
        &self,
        receiver_id: &AccountId,
        actions: Vec<Action>,
        wait_until: Option<TxExecutionStatus>,
    ) -> Result<FinalExecutionOutcomeView> {
        let signer = &*self.signer();
        self.client.send_tx(
            signer,
            receiver_id,
            actions,
            wait_until,
        ).await.map_err(|err| self.handle_transaction_error(err))
    }

    async fn process_transfer_action_internal(
        &self,
        receiver_id: AccountId,
        actions: Vec<Action>
    ) -> Result<TransactionResultResponse> {
        let execution = self.execute_actions(&receiver_id, actions, Some(TxExecutionStatus::ExecutedOptimistic)).await?;
        self.transaction_result_response(execution).await
    }

    async fn create_transfer_actions(
        &self,
        transfer_action_type: TransferActionType,
    ) -> Result<(AccountId, Vec<Action>)> {
        match transfer_action_type {
            TransferActionType::Native { user_address, amount_in_smallest_unit } => {
                let receiver_id = AccountId::from_str(&user_address).unwrap();
                let actions = vec![Action::Transfer(TransferAction {
                    deposit: amount_in_smallest_unit.to_u128().unwrap(),
                })];
                Ok((receiver_id, actions))
            }
            TransferActionType::FtTransfer { ft_contract_id, user_address, amount_in_smallest_unit } => {
                let receiver_id = AccountId::from_str(&user_address).unwrap();
                let args = json!({
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
                Ok((ft_contract_id, actions))
            }
        }
    }

    async fn create_signed_delegate_action(&self, receiver_id: AccountId, actions: Vec<Action>) -> Result<SignedDelegateAction> {
        let signer = &*self.signer();
        let relayer_account_id: AccountId = signer.account_id().clone();
        let public_key = signer.public_key();
        let (nonce, _block_hash, _) = self
            .client
            .fetch_nonce(&relayer_account_id, &public_key.clone())
            .await
            .unwrap();

        let delegate_action = DelegateAction {
            sender_id: relayer_account_id.clone(),
            receiver_id: receiver_id,
            actions: actions
                .into_iter()
                .map(|a| NonDelegateAction::try_from(a).unwrap())
                .collect(),
            nonce: nonce + 3,
            max_block_height: 2000000000 as BlockHeight,
            public_key: public_key.clone(),
        };

        let signable = SignableMessage::new(&delegate_action, SignableMessageType::DelegateAction);
        Ok(SignedDelegateAction {
            signature: signable.sign(signer),
            delegate_action,
        })
    }

    async fn transaction_result_response(
        &self,
        execution: FinalExecutionOutcomeView,
    ) -> Result<TransactionResultResponse> {
        let mut error_occurred = false;
        let mut error_details = Vec::new();
    
        for receipt_outcome in &execution.receipts_outcome {
            match &receipt_outcome.outcome.status {
                ExecutionStatusView::SuccessValue(_) => continue,
                ExecutionStatusView::Failure( error ) => {
                    let error_msg = format!("Transaction failed: {:#?}", error);

                    match error {
                        TxExecutionError::ActionError(TxActionError { kind, .. }) => {
                            match kind {
                                ActionErrorKind::DelegateActionInvalidNonce { .. } => {
                                    return Err(Error::CustomInvalidNonce);
                                },
                                _ => {
                                    tracing::error!("{}", error_msg);
                                    error_details.push(error_msg);
                                    error_occurred = true;
                                }
                            }
                        },
                        _ => {
                            tracing::error!("{}", error_msg);
                            error_details.push(error_msg);
                            error_occurred = true;
                        }
                    }
                },
                _ => {
                    let error_msg = "Transaction failed with an unknown error.".to_string();
                    tracing::error!("{}", error_msg);
                    error_details.push(error_msg);
                    error_occurred = true;
                }
            }
        }
        
        let mut res = TransactionResultResponse {
            message: String::new(),
            status: execution.status.clone(),
            receiver_id: execution.transaction.receiver_id.clone(),
            transaction_hash: execution.transaction.hash.clone(),
            has_errors: error_occurred,
            error_details,
        };

        if error_occurred {
            res.message = "Transaction encountered errors in receipt outcomes".to_string();
        } else {
            res.message = "Relayed and sent transaction".to_string();
            tracing::info!("Success message: \n{res:?}");
        }
        Ok(res)   
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

    pub async fn send_storage_deposit(
        &self,
        contract_id: AccountId,
        receiver_id: AccountId,
    ) -> Result<FinalExecutionOutcomeView>{
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
        self.execute_actions(&contract_id, actions, Some(TxExecutionStatus::ExecutedOptimistic)).await
    }
}

#[async_trait]
impl RpcClient for NearRpcManager {
    #[allow(unused)]
    async fn relay(
        &self,
        data: Vec<u8>,
    ) -> Result<TransactionResultResponse> {
        match SignedDelegateAction::try_from_slice(&data) {
            Ok(signed_delegate_action) => {
                let tx_result = self.process_signed_delegate_action(&signed_delegate_action, None).await?;
                Ok(tx_result)
            }
            Err(e) => {
                let err_msg = format!("Error deserializing payload data object: {e:?}");
                tracing::warn!("{err_msg}");
                Err(Error::InvalidEncodedSignedDelegateDeserialization { message: err_msg })
            }
        }
    }

    async fn process_transfer_action(
        &self,
        transfer_action_type: TransferActionType,
        is_delegated: bool,
    ) -> Result<TransactionResultResponse> {
        if is_delegated {
            let (receiver_id, actions) = self.create_transfer_actions(transfer_action_type).await?;
            let signed_delegate_action = self.create_signed_delegate_action(receiver_id, actions).await?;
            self.process_signed_delegate_action(&signed_delegate_action, None).await
        } else {
            let (receiver_id, actions) = self.create_transfer_actions(transfer_action_type).await?;
            self.process_transfer_action_internal(receiver_id, actions).await
        }
    }

    async fn process_signed_delegate_action(
        &self,
        signed_delegate_action: &SignedDelegateAction,
        _wait_until: Option<TxExecutionStatus>,
    ) -> Result<TransactionResultResponse> {
        tracing::debug!(
            "Deserialized SignedDelegateAction object: {:#?}",
            signed_delegate_action
        );
    
        self.validate_signed_delegate_action(signed_delegate_action.clone()).await?;
        let receiver_id: &AccountId = &signed_delegate_action.delegate_action.sender_id;
        let actions: Vec<Action> = vec![Action::Delegate(Box::new(signed_delegate_action.clone()))];
        self.process_transfer_action_internal(receiver_id.clone(), actions).await
    }
}