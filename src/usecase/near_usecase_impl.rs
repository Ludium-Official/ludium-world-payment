use std::sync::Arc;
use async_trait::async_trait;
use near_primitives::action::delegate::SignedDelegateAction;
use near_primitives::borsh::BorshDeserialize;
use near_primitives::views::{ExecutionStatusView, TxExecutionStatus};
use crate::adapter::output::near::rpc_client::NearRpcManager;
use crate::domain::model::near::TransactionResultResponse;
use crate::port::output::rpc_client::RpcClient;

use super::error::{Result, Error};
use super::utrait::near_usecase::NearUsecase;

#[derive(Clone, Debug)]
pub struct NearUsecaseImpl;

#[async_trait]
impl NearUsecase for NearUsecaseImpl {
    #[allow(unused)]
    async fn relay(
        &self,
        near_rpc_manager: Arc<NearRpcManager>,
        data: Vec<u8>,
    ) -> Result<TransactionResultResponse> {
        match SignedDelegateAction::try_from_slice(&data) {
            Ok(signed_delegate_action) => {
                let tx_result = self.process_signed_delegate_action(Arc::clone(&near_rpc_manager), &signed_delegate_action, None).await?;
                Ok(tx_result)
            }
            Err(e) => {
                let err_msg = format!("Error deserializing payload data object: {e:?}");
                tracing::warn!("{err_msg}");
                Err(Error::InvalidEncodedSignedDelegateDeserialization { message: err_msg })
            }
        }
    }

    async fn process_signed_delegate_action(
        &self,
        near_rpc_manager: Arc<NearRpcManager>,
        signed_delegate_action: &SignedDelegateAction,
        _wait_until: Option<TxExecutionStatus>,
    ) -> Result<TransactionResultResponse> {
        let result = near_rpc_manager.send_transaction(signed_delegate_action.clone()).await;
        match result {
            Ok(execution) => {
                let mut error_occurred = false;
                let mut error_details = Vec::new();
    
                for receipt_outcome in &execution.receipts_outcome {
                    match &receipt_outcome.outcome.status {
                        ExecutionStatusView::SuccessValue(_) => continue,
                        ExecutionStatusView::Failure(error) => {
                            let error_msg = format!("Transaction failed: {:#?}", error);
                            tracing::error!("{}", error_msg);
                            error_details.push(error_msg);
                            error_occurred = true;
                        }
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
            Err(err_msg) => {
                tracing::error!("Error message: \n{err_msg}");
                Err(Error::InternalServerError {
                    message: err_msg.to_string(),
                })
            }
        }
    }
}
