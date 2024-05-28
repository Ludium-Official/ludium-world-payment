use std::sync::Arc;
use async_trait::async_trait;
use near_primitives::action::delegate::SignedDelegateAction;
use near_primitives::borsh::BorshDeserialize;
use near_primitives::views::{FinalExecutionStatus, TxExecutionStatus};
use crate::domain::model::near::TransactionResultResponse;
use crate::port::output::rpc_client::RpcClient;

use super::error::{Result, Error};
use super::utrait::near_usecase::NearUsecase;

#[derive(Clone, Debug)]
pub struct NearUsecaseImpl;

#[async_trait]
impl NearUsecase for NearUsecaseImpl {
    async fn relay(
        &self,
        near_rpc_manager: &Arc<dyn RpcClient>,
        data: Vec<u8>,
    ) -> Result<TransactionResultResponse> {
        tracing::debug!("[handler] relay");
    
        match SignedDelegateAction::try_from_slice(&data) {
            Ok(signed_delegate_action) => {
                let tx_result = process_signed_delegate_action(near_rpc_manager, &signed_delegate_action, None).await?;
                Ok(tx_result)
            }
            Err(e) => {
                let err_msg = format!("Error deserializing payload data object: {e:?}");
                tracing::warn!("{err_msg}");
                Err(Error::InvalidEncodedSignedDelegateDeserialization { message: err_msg })
            }
        }
    }
}

pub async fn relay2(
    near_rpc_manager: &Arc<dyn RpcClient>,
    data: Vec<u8>,
) -> Result<TransactionResultResponse> {
    tracing::debug!("[handler] relay");

    match SignedDelegateAction::try_from_slice(&data) {
        Ok(signed_delegate_action) => {
            let tx_result = process_signed_delegate_action(near_rpc_manager, &signed_delegate_action, None).await?;
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
    near_rpc_manager: &Arc<dyn RpcClient>,
    signed_delegate_action: &SignedDelegateAction,
    _wait_until: Option<TxExecutionStatus>,
) -> Result<TransactionResultResponse> {
    let result = near_rpc_manager.send_transaction(signed_delegate_action.clone()).await;
    match result {
        Ok(execution) => {
            tracing::debug!("Transaction execution\n{execution:?}");

            if let FinalExecutionStatus::Failure(_) = execution.status {
                let res = TransactionResultResponse {
                    message: "Transaction Failed".to_string(),
                    status: execution.status.clone(),
                    receiver_id: execution.transaction.receiver_id.clone(),
                    transaction_hash: execution.transaction.hash.clone(),
                };

                Ok(res)
            }else {
                let res = TransactionResultResponse {
                    message: "Relayed and sent transaction".to_string(),
                    status: execution.status.clone(),
                    receiver_id: execution.transaction.receiver_id.clone(),
                    transaction_hash: execution.transaction.hash.clone(),
                };
                tracing::info!("Success message: \n{res:?}");
                Ok(res)   
            }
        }
        Err(err_msg) => {
            tracing::error!("Error message: \n{err_msg}");
            Err(Error::InternalServerError {
                message: err_msg.to_string(),
            })
        }
    }
}


