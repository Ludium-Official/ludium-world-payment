use std::sync::Arc;
use async_trait::async_trait;
use near_primitives::action::delegate::SignedDelegateAction;
use near_primitives::borsh::BorshDeserialize;
use near_primitives::views::{ExecutionStatusView, TxExecutionStatus};
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

async fn process_signed_delegate_action(
    near_rpc_manager: &Arc<dyn RpcClient>,
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


#[cfg(test)]
mod tests {
    use crate::adapter::output::near::rpc_client::NearRpcManager;
    use crate::config::near::KeyRotatingSignerWrapper;
    use crate::usecase::utrait::near_usecase::NearUsecase;
    use crate::config::config;

    use super::*;
    use near_primitives::action::delegate::NonDelegateAction;
    use near_primitives::action::Action;
    use near_primitives::borsh::{self};
    
    use near_primitives::views::FinalExecutionStatus;
    use near_primitives::signable_message::{SignableMessage, SignableMessageType};
    use near_primitives::transaction::{FunctionCallAction, TransferAction};
    use near_primitives::types::Balance;
    use near_primitives::types::BlockHeight;
    use near_primitives::types::AccountId;
    
    use near_primitives::action::delegate::DelegateAction;
    use std::str::FromStr;
    use near_crypto::{InMemorySigner, PublicKey, SecretKey};
    use std::sync::Arc;
    use serial_test::serial;

    async fn init_setting(
        amount: u128,
        whitelisted_contract: Vec<String>,
        whitelisted_senders: Vec<String>
    ) -> (
        AccountId,
        PublicKey,
        SecretKey,
        Vec<Action>,
        u64,
        Arc<dyn RpcClient>,
    ) {
        let receiver_id: AccountId = "won999.testnet".parse().unwrap();
        let actions = create_function_call(
            "ft_transfer".to_string(),
            serde_json::json!({
                "receiver_id": receiver_id.to_string(),
                "amount": amount.to_string()
            })
        );

        let account_id: AccountId = "nomnomnom.testnet".parse().unwrap();
        let public_key: PublicKey =
            PublicKey::from_str("ed25519:89GtfFzez3opomVpwa7i4m3nptHtc7Ha514XHMWszQtL").unwrap();
        let secret_key: SecretKey = SecretKey::from_str("ed25519:WYuyKVQHE3rJQYRC3pRGV56o1qEtA1PnMYPDEtroc5kX4A4mWrJwF7XkzGe7JWNMABbtY4XFDBJEzgLyfPkwCzp").unwrap();
        let signer = InMemorySigner::from_secret_key(account_id.clone(), secret_key.clone());

        let config = config().await;
        let near_rpc_manager = Arc::new(NearRpcManager::new(
            config.get_near_network_config().rpc_client(),
            KeyRotatingSignerWrapper::from_signers(vec![signer]),
            whitelisted_contract,
            whitelisted_senders,
        ));

        let (nonce, _block_hash, _) = near_rpc_manager
            .client
            .fetch_nonce(&account_id.clone(), &public_key.clone())
            .await
            .unwrap();

        let arc_mock_rpc_client: Arc<dyn RpcClient> = near_rpc_manager;

        (account_id, public_key, secret_key, actions, nonce, arc_mock_rpc_client)
    }

    fn create_usdt_trnasfer_signed_delegate_action(
        sender_account_id: AccountId,
        public_key: PublicKey,
        secret_key: SecretKey,
        actions: Option<Vec<Action>>,
        nonce: Option<u64>,
    ) -> SignedDelegateAction {
        let signer = InMemorySigner{
            account_id: sender_account_id.clone(), public_key: public_key.clone(), secret_key: secret_key};
        let receiver_account_id: AccountId = "tt_local.testnet".parse().unwrap();

        let mut actions_vec = vec![Action::Transfer(TransferAction {
            deposit: 0.00000001 as Balance,
        })];

        if actions.is_some() {
            actions_vec = actions.unwrap();
        }

        let delegate_action = DelegateAction {
            sender_id: sender_account_id.clone(),
            receiver_id: receiver_account_id,
            actions: actions_vec
                .into_iter()
                .map(|a| NonDelegateAction::try_from(a).unwrap())
                .collect(),
            nonce: nonce.unwrap_or(0) + 2,
            max_block_height: 2000000000 as BlockHeight,
            public_key,
        };

        let signable = SignableMessage::new(&delegate_action, SignableMessageType::DelegateAction);
        SignedDelegateAction {
            signature: signable.sign(&signer),
            delegate_action,
        }
    }

    fn create_function_call(method_name: String, args: serde_json::Value) -> Vec<Action>{
        let function_call_action = FunctionCallAction {
            method_name: method_name,
            args: args.to_string().into_bytes(),
            gas: 100_000_000_000_000, // 100 Tgas
            deposit: 1, // 1 yoctoNEAR for the function call
        };
        vec![Action::FunctionCall(Box::new(function_call_action))]
    }

    #[serial]
    #[tokio::test]
    async fn test_relay_success() -> Result<()> {
        let amount: u128 = 1 * 10u128.pow(1); // 0.00001 USDT, assuming USDC has 6 decimal places
        let (account_id, public_key, secret_key, actions, nonce, arc_mock_rpc_client) 
            = init_setting(amount, vec![], vec![]).await;

        let signed_delegate_action: SignedDelegateAction = create_usdt_trnasfer_signed_delegate_action(
            account_id.clone(), public_key.clone(), secret_key.clone(), Some(actions), Some(nonce));
        let serialized_signed_delegate_action: Vec<u8> = borsh::to_vec(&signed_delegate_action).unwrap();
            
        let near_usecase_impl = NearUsecaseImpl;
        let result = near_usecase_impl
            .relay(&arc_mock_rpc_client, serialized_signed_delegate_action)
            .await;

        println!("{:?}", result);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.receiver_id, account_id);
        assert_eq!(response.status, FinalExecutionStatus::SuccessValue(vec![]));
        assert!(!response.has_errors);

        Ok(())
    }

    #[serial]
    #[tokio::test]
    async fn test_relay_failure_balance_insufficient() -> Result<()> {
        let amount: u128 = 50000 * 10u128.pow(6); // 50000 USDT, assuming USDC has 6 decimal places
        let (account_id, public_key, secret_key, actions, nonce, arc_mock_rpc_client) 
            = init_setting(amount, vec![], vec![]).await;

        let signed_delegate_action: SignedDelegateAction = create_usdt_trnasfer_signed_delegate_action(
            account_id.clone(), public_key.clone(), secret_key.clone(), Some(actions), Some(nonce));
        let serialized_signed_delegate_action: Vec<u8> = borsh::to_vec(&signed_delegate_action).unwrap();

        let near_usecase_impl = NearUsecaseImpl;
        let result = near_usecase_impl
            .relay(&arc_mock_rpc_client, serialized_signed_delegate_action)
            .await;

        println!("{:?}", result);
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.receiver_id, account_id);
        assert!(response.has_errors);
        assert!(response.error_details[0].contains("u128::from(self.ft_balance_of(sender_id)) >= u128::from(amount)"));

        Ok(())
    }

    #[tokio::test]
    async fn test_relay_invalid_data_whitelisted_contract() -> Result<()> {
        let amount: u128 = 1 * 10u128.pow(1); // 0.00001 USDT, assuming USDC has 6 decimal places
        let whitelisted_contract = vec!["tt_local.testnet".to_string()];
        let (account_id, public_key, secret_key, actions, nonce, arc_mock_rpc_client) 
            = init_setting(amount, whitelisted_contract, vec![]).await;
        
        let signed_delegate_action: SignedDelegateAction = create_usdt_trnasfer_signed_delegate_action(
                account_id.clone(), public_key.clone(), secret_key.clone(), Some(actions), Some(nonce));
        let serialized_signed_delegate_action: Vec<u8> = borsh::to_vec(&signed_delegate_action).unwrap();

        let near_usecase_impl = NearUsecaseImpl;
        let result = near_usecase_impl
            .relay(&arc_mock_rpc_client, serialized_signed_delegate_action)
            .await;

        println!("{:?}", result);
        assert!(result.is_err());
        Ok(())
    }

    #[tokio::test]
    async fn test_relay_invalid_data_whitelisted_senders() -> Result<()> {
        let amount: u128 = 1 * 10u128.pow(1); // 0.00001 USDT, assuming USDC has 6 decimal places
        let whitelisted_senders = vec!["hello.testnet".to_string()];
        let (account_id, public_key, secret_key, actions, nonce, arc_mock_rpc_client) 
            = init_setting(amount, vec![], whitelisted_senders).await;
        
        let signed_delegate_action: SignedDelegateAction = create_usdt_trnasfer_signed_delegate_action(
                account_id.clone(), public_key.clone(), secret_key.clone(), Some(actions), Some(nonce));
        let serialized_signed_delegate_action: Vec<u8> = borsh::to_vec(&signed_delegate_action).unwrap();

        let near_usecase_impl = NearUsecaseImpl;
        let result = near_usecase_impl
            .relay(&arc_mock_rpc_client, serialized_signed_delegate_action)
            .await;

        println!("{:?}", result);
        assert!(result.is_err());
        Ok(())
    }

}
