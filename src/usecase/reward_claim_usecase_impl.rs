use std::sync::Arc;
use async_trait::async_trait;
use bigdecimal::{ToPrimitive, BigDecimal};
// use deadpool_diesel::postgres::Object;
// use near_crypto::{InMemorySigner, PublicKey, SecretKey};
// use near_fetch::signer::ExposeAccountId;
// use near_primitives::{action::TransferAction, types::Balance, views::FinalExecutionOutcomeView};
// use serde_json::json;
use uuid::Uuid;
use crate::{
    adapter::output::near::rpc_client::NearRpcManager, domain::model::{
        coin::CoinType, near::{TransactionResultResponse, TransferActionType}, reward_claim::{
            CombinedRewardClaimResponse, NewRewardClaim, NewRewardClaimPayload, RewardClaimStatus
        }, reward_claim_detail::NewRewardClaimDetail
    }, port::output::{
        coin_network_repository::CoinNetworkRepository, reward_claim_repository::RewardClaimRepository, rpc_client::RpcClient, DbManager
    }
};
use super::{error::{Error, Result}, utrait::near_usecase::NearUsecase};
use super::utrait::reward_claim_usecase::RewardClaimUsecase;
use std::str::FromStr;
// use near_primitives::views::TxExecutionStatus;
// use near_primitives::signable_message::SignableMessageType;
// use near_primitives::signable_message::SignableMessage;
// use near_primitives::types::BlockHeight;
// use near_primitives::action::delegate::NonDelegateAction;
// use near_primitives::action::delegate::DelegateAction;
use near_primitives::types::AccountId;
// use near_primitives::action::Action;

// use base64::engine::general_purpose::STANDARD_NO_PAD as BASE64_ENGINE;
// use base64::Engine;

pub struct RewardClaimUsecaseImpl<D: DbManager, R: RewardClaimRepository, C: CoinNetworkRepository, U: NearUsecase> {
    db_manager: Arc<D>,
    reward_claim_repo: Arc<R>,
    coin_network_repo: Arc<C>,
    near_usecase: Arc<U>,
    near_rpc_manager: Arc<NearRpcManager>,
}

impl<D, R, C, U> RewardClaimUsecaseImpl<D, R, C, U>
where
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
    U: NearUsecase + Send + Sync,
{
    pub fn new(db_manger: Arc<D>, reward_claim_repo: Arc<R>, coin_network_repo: Arc<C>, near_usecase: Arc<U>, near_rpc_manager: Arc<NearRpcManager>) -> Self {
        Self {
            db_manager: db_manger,
            reward_claim_repo,
            coin_network_repo,
            near_usecase,
            near_rpc_manager,
        }
    }
}

#[async_trait]
impl<D, R, C, U> RewardClaimUsecase for RewardClaimUsecaseImpl<D, R, C, U>
where 
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
    U: NearUsecase + Send + Sync,
{
    // actually, at this time don't need to meta_tx logic, just send_tx is good. but for the future, I will keep this relayer code
    async fn create_reward_claim(&self, user_id: Uuid, payload: NewRewardClaimPayload) -> Result<CombinedRewardClaimResponse> {
        tracing::debug!("create_reward_claim");
        let db_manager = &self.db_manager;
        
        let (coin_network, coin, network) = self.coin_network_repo
            .get_with_coin_and_network(
                self.db_manager.get_connection().await?.into(),
                payload.coin_network_id
            )
            .await
            .map_err(|_| Error::CoinNetworkIdNotFound { id: payload.coin_network_id.to_string() })?;

        if self.reward_claim_repo.get_by_mission_and_user(db_manager.get_connection().await?.into(), payload.mission_id, payload.user_id).await.is_ok() {
            return Err(Error::RewardClaimDuplicate { mission_id: payload.mission_id.to_string(), user_id: payload.user_id.to_string() });
        }

        let scale_factor = BigDecimal::from_str(&format!("1e{}", coin.decimals)).expect("Invalid decimal format");
        let amount_decimal = BigDecimal::from_str(&payload.amount).expect("Invalid amount format");
        let amount_in_smallest_unit = (amount_decimal * scale_factor).to_u128().ok_or(Error::InvalidAmountConversion)?;

        let tx_result_response: Result<TransactionResultResponse>;
        match coin.coin_type {
            CoinType::Native => {
                let signed_delegate_action = self.near_rpc_manager.create_transfer_signed_delegate_action(
                    TransferActionType::Native {
                        user_address: payload.user_address.to_string(),
                        amount_in_smallest_unit,
                    }
                ).await?;

                tx_result_response =  self.near_usecase
                    .process_signed_delegate_action(Arc::clone(&self.near_rpc_manager), &signed_delegate_action, None).await;
            }
            CoinType::FT => {
                tracing::debug!("send delegate tx");
                let contract_address = coin_network
                    .contract_address.as_ref().ok_or_else(|| Error::InternalServerError { message: "contract_address is empty".to_string() })?;

                // todo: if user_address is registered, skip this storage_deposit step
                let deposit_result = self.near_rpc_manager.send_storage_deposit(
                    AccountId::from_str(contract_address).unwrap(),
                    AccountId::from_str(payload.user_address.as_str()).unwrap(),
                ).await?;
                println!("deposit_result: {:?}", deposit_result);

                let signed_delegate_action = self.near_rpc_manager.create_transfer_signed_delegate_action(
                    TransferActionType::FtTransfer {
                        ft_contract_id: AccountId::from_str(contract_address).unwrap(),
                        user_address: payload.user_address.to_string(),
                        amount_in_smallest_unit,
                    }
                ).await?;
                signed_delegate_action.verify();

                tx_result_response =  self.near_usecase
                    .process_signed_delegate_action(Arc::clone(&self.near_rpc_manager), &signed_delegate_action, None).await;
                
            }
            _ => {
                return Err(Error::CoinTypeNotSupported { coin_type: coin.coin_type.to_string() });
            }
        }

        match tx_result_response {
            Ok(tx_result) => {
                let reward_claim_status: RewardClaimStatus;
                if tx_result.has_errors {
                    reward_claim_status = RewardClaimStatus::TransactionFailed;
                }else {
                    reward_claim_status = RewardClaimStatus::TransactionApproved;
                }

                let new_reward_claim = NewRewardClaim {
                    id: Uuid::new_v4(),
                    mission_id: payload.mission_id,
                    coin_network_id: payload.coin_network_id,
                    reward_claim_status: reward_claim_status,
                    amount: amount_in_smallest_unit as i64,
                    user_id: payload.user_id,
                    user_address: payload.user_address,
                };

                let reward_claim = self.reward_claim_repo.insert(db_manager.get_connection().await?.into(), new_reward_claim).await?;
                let new_reward_claim_detail = NewRewardClaimDetail {
                    id: Uuid::new_v4(),
                    reward_claim_id: reward_claim.id,
                    transaction_hash: tx_result.transaction_hash.to_string(),
                    sended_user_id: user_id,
                    sended_user_address: tx_result.receiver_id.to_string(),
                };

                let claim_detail = self.reward_claim_repo.insert_detail(db_manager.get_connection().await?.into(), new_reward_claim_detail).await?;
                Ok(CombinedRewardClaimResponse::from((reward_claim, claim_detail, coin_network, coin, network)))
            }
            Err(e) => {
                Err(Error::InternalServerError { message: e.to_string() })
            }
        }
    }
}

