use std::future::Future;
use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use near_primitives::action::delegate::DelegateAction;
use near_primitives::action::Action;
use near_primitives::borsh::BorshDeserialize;
use near_primitives::types::AccountId;
use near_primitives::{action::delegate::SignedDelegateAction, views::TxExecutionStatus};
use near_primitives::views::{FinalExecutionOutcomeView, FinalExecutionStatus};
use crate::domain::model::near::{TransactionResult, TransactionResultResponse};
use crate::AppState;
use serde_json::json;
use super::error::Error;
use crate::config::near::ROTATING_SIGNER;

async fn relay(State(state): State<Arc<AppState>>, data: Json<Vec<u8>>) -> Result<Json<TransactionResultResponse>, Error> {
    tracing::debug!("[handler] relay");

    match SignedDelegateAction::try_from_slice(&data.0) {
        Ok(signed_delegate_action) => {
            let tx_result = process_signed_delegate_action(state.as_ref(), &signed_delegate_action, None).await?;
            Ok(Json(tx_result))
        }
        Err(e) => {
            let err_msg = format!("Error deserializing payload data object: {e:?}");
            tracing::warn!("{err_msg}");
            Err(Error::InvalidEncodedSignedDelegateDeserialization {
                message: err_msg,
            })
        }
    }
}

async fn process_signed_delegate_action(
    state: &AppState,
    signed_delegate_action: &SignedDelegateAction,
    wait_until: Option<TxExecutionStatus>,
) -> Result<TransactionResultResponse, Error> {
    let wait_until_param = wait_until.unwrap_or(TxExecutionStatus::ExecutedOptimistic);
    let result = filter_and_send_signed_delegate_action(
        state,
        signed_delegate_action.clone(),
    ).await;

    println!("Result\n{result:?}");

    match result {
        Ok(execution) => {
            println!("Execution\n {:?}", execution);
            println!("ExecutionStatus: {:?}", execution.status.clone());
            println!("ExecutionAction: {:?}", execution.transaction.actions[0]);
            println!("ExecutionActionReceiver: {:?}", execution.transaction.receiver_id);
            println!("ExecutionActionTxHash: {:?}", execution.transaction.hash);

            
            if let FinalExecutionStatus::Failure(_) = execution.status {
                // tracing::error!("Error message: \n{status_msg:?}");
                println!("Error Txhash: {:?}", execution.transaction.hash);
                // return Err(Error::RelayError {
                //     message: status_msg.message.clone(),
                // });

                let status_msg = TransactionResultResponse {
                    message: "Transaction Failed".to_string(),
                    status: execution.status.clone(),
                    receiver_id: execution.transaction.receiver_id.clone(),
                    transaction_hash: execution.transaction.hash.clone(),
                };

                Ok(status_msg)
            }else {
                let status_msg = TransactionResultResponse {
                    message: "Relayed and sent transaction".to_string(),
                    status: execution.status.clone(),
                    receiver_id: execution.transaction.receiver_id.clone(),
                    transaction_hash: execution.transaction.hash.clone(),
                };
                tracing::info!("Success message: \n{status_msg:?}");
                Ok(status_msg)   
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


async fn filter_and_send_signed_delegate_action(
    state: &AppState,
    signed_delegate_action: SignedDelegateAction,
) -> Result<FinalExecutionOutcomeView, Error>
{
    tracing::debug!(
        "Deserialized SignedDelegateAction object: {:#?}",
        signed_delegate_action
    );

    // TODO: Implement validation
    // let validation_result: Result<(), Error> =
    //     validate_signed_delegate_action(state, &signed_delegate_action);
    // validation_result?;

    let receiver_id: &AccountId = &signed_delegate_action.delegate_action.sender_id;
    let actions: Vec<Action> = vec![Action::Delegate(Box::new(signed_delegate_action.clone()))];

    let execution = state
        .near_rpc_manager
        .client
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
            Error::InternalServerError {
                message: err_msg,
            }
        })?;

    let status = &execution.status;
    let response_msg = match status {
        FinalExecutionStatus::Failure(_) => "Error sending transaction",
        _ => "Relayed and sent transaction",
    };

    if let FinalExecutionStatus::Failure(_) = status {
        Err(Error::InternalServerError {
            message: "Fail Execution".to_string()
        })
    } else {
        Ok(execution)
    }
}

// endregion: --- REST Handlers

// region: --- meta_tx real test
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::adapter::output::near::rpc_client::NearRpcManager;
//     use crate::adapter::output::persistence::db::_dev_utils;
//     use crate::adapter::output::persistence::db::postgres::coin_network_repository_impl::PostgresCoinNetworkRepository;
//     use crate::adapter::output::persistence::db::postgres::coin_repository_impl::PostgresCoinRepository;
//     use crate::adapter::output::persistence::db::postgres::network_repository_impl::PostgresNetworkRepository;
//     use crate::adapter::output::persistence::db::postgres::reward_claim_repository_impl::PostgresRewardClaimRepository;
//     use crate::adapter::output::persistence::db::postgres::{user_repository_impl::PostgresUserRepository, PostgresDbManager};
//     use crate::config::config;
//     use crate::usecase::near_usecase_impl::NearUsecaseImpl;
//     use crate::usecase::reward_claim_usecase_impl::RewardClaimUsecaseImpl;
//     use crate::usecase::utrait::reward_claim_usecase::RewardClaimUsecase;

//     use axum::response::Response;
//     use axum::{
//         extract::{Json, State},
//         http::StatusCode,
//     };
//     use std::str::FromStr;

//     use near_crypto::KeyType::ED25519;
//     use near_crypto::{InMemorySigner, PublicKey, SecretKey, Signature, Signer};

//     use near_primitives::account::{AccessKey, AccessKeyPermission};
//     use near_primitives::action::delegate::{
//         DelegateAction, NonDelegateAction, SignedDelegateAction,
//     };
//     use near_primitives::borsh;
//     use near_primitives::signable_message::{SignableMessage, SignableMessageType};
//     use near_primitives::transaction::{Action, AddKeyAction, FunctionCallAction, TransferAction};
//     use near_primitives::types::Balance;
//     use near_primitives::types::{BlockHeight, Nonce};

//     async fn create_app_state() -> AppState {
//         let config = config().await;

//         let db_manager = Arc::new(_dev_utils::init_test().await);
//         let user_repo = Arc::new(PostgresUserRepository);
//         let coin_repo = Arc::new(PostgresCoinRepository);
//         let network_repo = Arc::new(PostgresNetworkRepository);
//         let coin_network_repo = Arc::new(PostgresCoinNetworkRepository);
//         let reward_claim_repo = Arc::new(PostgresRewardClaimRepository);
//         let near_rpc_manager = Arc::new(NearRpcManager::new(config.near_network_config.rpc_client()));
//         let near_usecase = Arc::new(NearUsecaseImpl);
//         let reward_claim_usecase: Arc<dyn RewardClaimUsecase + Send + Sync> = Arc::new(RewardClaimUsecaseImpl::new(
//             Arc::clone(&db_manager),
//             Arc::clone(&reward_claim_repo),
//             Arc::clone(&coin_network_repo),
//             Arc::clone(&near_usecase),
//             Arc::clone(&near_rpc_manager),
//         ));

//         AppState {
//             db_manager: Arc::clone(&db_manager),
//             user_repo: Arc::clone(&user_repo),
//             coin_repo: Arc::clone(&coin_repo),
//             network_repo: Arc::clone(&network_repo),
//             coin_network_repo: Arc::clone(&coin_network_repo),
//             reward_claim_repo: Arc::clone(&reward_claim_repo),
//             near_usecase: Arc::clone(&near_usecase),
//             reward_claim_usecase: Arc::clone(&reward_claim_usecase),
//             near_rpc_manager: Arc::clone(&near_rpc_manager),
//         }
//     }

//     fn convert_app_state_to_arc_app_state(app_state: AppState) -> State<Arc<AppState>> {
//         let shared_state = Arc::new(app_state);
//         State(shared_state.clone())
//     }

//     fn create_signed_delegate_action(
//         sender_id: Option<&str>,
//         receiver_id: Option<&str>,
//         actions: Option<Vec<Action>>,
//         nonce: Option<u64>,
//     ) -> SignedDelegateAction {
//         let seed: String =
//             "nuclear egg couch off antique brave cake wrap orchard snake prosper one".to_string();
//         let mut sender_account_id: AccountId = "relayer_test0.testnet".parse().unwrap();
//         let public_key = PublicKey::from_seed(ED25519, &seed.clone());
//         let signer = InMemorySigner::from_seed(sender_account_id.clone(), ED25519, &seed.clone());

//         let mut receiver_account_id: AccountId = "relayer_test1.testnet".parse().unwrap();

//         let mut actions_vec = vec![Action::Transfer(TransferAction {
//             deposit: 0.00000001 as Balance,
//         })];

//         if sender_id.is_some() {
//             sender_account_id = sender_id.unwrap().parse().unwrap();
//         }
//         if receiver_id.is_some() {
//             receiver_account_id = receiver_id.unwrap().parse().unwrap();
//         }
//         if actions.is_some() {
//             actions_vec = actions.unwrap();
//         }

//         let delegate_action = DelegateAction {
//             sender_id: sender_account_id.clone(),
//             receiver_id: receiver_account_id,
//             actions: actions_vec
//                 .into_iter()
//                 .map(|a| NonDelegateAction::try_from(a).unwrap())
//                 .collect(),
//             nonce: nonce.unwrap_or(0),
//             max_block_height: 2000000000 as BlockHeight,
//             public_key,
//         };

//         let signable = SignableMessage::new(&delegate_action, SignableMessageType::DelegateAction);
//         SignedDelegateAction {
//             signature: signable.sign(&signer),
//             delegate_action,
//         }
//     }


//     #[ignore]
//     #[tokio::test]
//     async fn test_relay() {
//         let app_state = create_app_state().await;
//         let axum_state: State<Arc<AppState>> = convert_app_state_to_arc_app_state(app_state);
//         let account_id: AccountId = "relayer_test0.testnet".parse().unwrap();
//         let public_key: PublicKey =
//             PublicKey::from_str("ed25519:AMypJZjcMYwHCx2JFSwXAPuygDS5sy1vRNc2aoh3EjTN").unwrap();

//         let (nonce, _block_hash, _) = &axum_state
//             .near_rpc_manager
//             .client
//             .fetch_nonce(&account_id, &public_key)
//             .await
//             .unwrap();

//         let signed_delegate_action = create_signed_delegate_action(None, None, None, Some(*nonce));
//         assert!(signed_delegate_action.verify());

//         let serialized_signed_delegate_action = borsh::to_vec(&signed_delegate_action).unwrap();
//         let json_payload = Json(serialized_signed_delegate_action);

//         let response = relay(axum_state, json_payload).await.unwrap();

//         println!("----------------------------");
//         println!("Response: {response:?}");
//     }


//     use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64_ENGINE;
//     use base64::Engine;

//     fn create_usdt_trnasfer_signed_delegate_action(
//         actions: Option<Vec<Action>>,
//         nonce: Option<u64>,
//     ) -> SignedDelegateAction {
//         let mut sender_account_id: AccountId = "nomnomnom.testnet".parse().unwrap();
//         let public_key: PublicKey =
//             PublicKey::from_str("ed25519:89GtfFzez3opomVpwa7i4m3nptHtc7Ha514XHMWszQtL").unwrap();
//         let secret_key: SecretKey = SecretKey::from_str("ed25519:WYuyKVQHE3rJQYRC3pRGV56o1qEtA1PnMYPDEtroc5kX4A4mWrJwF7XkzGe7JWNMABbtY4XFDBJEzgLyfPkwCzp").unwrap();

//         let signer = InMemorySigner{
//             account_id: sender_account_id.clone(), public_key: public_key.clone(), secret_key: secret_key};
//         let receiver_account_id: AccountId = "tt_local.testnet".parse().unwrap();

//         let mut actions_vec = vec![Action::Transfer(TransferAction {
//             deposit: 0.00000001 as Balance,
//         })];

//         if actions.is_some() {
//             actions_vec = actions.unwrap();
//         }

//         let delegate_action = DelegateAction {
//             sender_id: sender_account_id.clone(),
//             receiver_id: receiver_account_id,
//             actions: actions_vec
//                 .into_iter()
//                 .map(|a| NonDelegateAction::try_from(a).unwrap())
//                 .collect(),
//             nonce: nonce.unwrap_or(0) + 2,
//             max_block_height: 2000000000 as BlockHeight,
//             public_key,
//         };

//         let signable = SignableMessage::new(&delegate_action, SignableMessageType::DelegateAction);
//         SignedDelegateAction {
//             signature: signable.sign(&signer),
//             delegate_action,
//         }
//     }

//     // #[ignore]
//     #[tokio::test]
//     async fn test_usdc_transfer()  {
//         let app_state = create_app_state().await;
//         let axum_state: State<Arc<AppState>> = convert_app_state_to_arc_app_state(app_state);
//         let account_id: AccountId = "nomnomnom.testnet".parse().unwrap();
//         let public_key: PublicKey =
//             PublicKey::from_str("ed25519:89GtfFzez3opomVpwa7i4m3nptHtc7Ha514XHMWszQtL").unwrap();

//         // Parameters for USDC transfer
//         let usdc_contract_id = "tt_local.testnet";
//         let amount: u128 = 5 * 10u128.pow(2); //  0.0005 USDT, assuming USDC has 6 decimal places
//         let mut receiver_id: AccountId = "won999.testnet".parse().unwrap();
        
//         let args: serde_json::Value = json!({
//             "receiver_id": receiver_id.to_string(),
//             "amount": amount.to_string()
//         });
//         let args_base64 = BASE64_ENGINE.encode(args.to_string());

//         let function_call_action = FunctionCallAction {
//             method_name: "ft_transfer".to_string(),
//             args: args.to_string().into_bytes(),
//             gas: 100_000_000_000_000, // 100 Tgas
//             deposit: 1, // 1 yoctoNEAR for the function call
//         };

//         let actions = vec![Action::FunctionCall(Box::new(function_call_action))];
//         // Create `SignedDelegateAction`
//         let (nonce, block_hash, _) = &axum_state
//             .near_rpc_manager
//             .client
//             .fetch_nonce(&account_id, &public_key)
//             .await
//             .unwrap();

//         let signed_delegate_action = create_usdt_trnasfer_signed_delegate_action(Some(actions), Some(*nonce));
//         assert!(signed_delegate_action.verify());

//         let serialized_signed_delegate_action = borsh::to_vec(&signed_delegate_action).unwrap();
//         println!("serialized_signed_delegate_action: {:?}", serialized_signed_delegate_action);
//         let json_payload = Json(serialized_signed_delegate_action);

//         println!("json_payload: {:?}", json_payload);

//         // Call the `relay` function
//         // let response = relay(axum_state, json_payload).await.unwrap();

//         // println!("----------------------------");
//         // println!("Response: {response:?}");
//     }
// }

// endregion: --- meta_tx real test