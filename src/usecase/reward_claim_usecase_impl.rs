use std::{collections::{HashMap, HashSet}, sync::Arc};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use crate::{
    adapter::output::near::{retry_async, rpc_client::NearRpcManager, MAX_RETRY_COUNT, RETRY_DELAY}, domain::model::{
        coin::{Coin, CoinType}, coin_network::CoinNetwork, near::{TransactionResultResponse, TransferActionType}, network::Network, reward_claim::{
            CombinedRewardClaimResponse, NewRewardClaim, NewRewardClaimPayload, RewardClaim, RewardClaimStatus
        }, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}
    }, port::output::{
        coin_network_repository::CoinNetworkRepository, reward_claim_repository::RewardClaimRepository, rpc_client::RpcClient, DbManager
    }
};
use super::error::{Error, Result};
use super::utrait::reward_claim_usecase::RewardClaimUsecase;
use std::str::FromStr;
use near_primitives::types::AccountId;

pub struct RewardClaimUsecaseImpl<D: DbManager, R: RewardClaimRepository, C: CoinNetworkRepository> {
    db_manager: Arc<D>,
    reward_claim_repo: Arc<R>,
    coin_network_repo: Arc<C>,
    near_rpc_manager: Arc<NearRpcManager>,
}

impl<D, R, C> RewardClaimUsecaseImpl<D, R, C>
where
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
{
    pub fn new(db_manger: Arc<D>, reward_claim_repo: Arc<R>, coin_network_repo: Arc<C>, near_rpc_manager: Arc<NearRpcManager>) -> Self {
        Self {
            db_manager: db_manger,
            reward_claim_repo,
            coin_network_repo,
            near_rpc_manager,
        }
    }
}

#[async_trait]
impl<D, R, C> RewardClaimUsecase for RewardClaimUsecaseImpl<D, R, C,>
where 
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
{
    async fn get_me_reward_claim(&self, user_id: Uuid) -> Result<Vec<CombinedRewardClaimResponse>> {
        let db_manager = &self.db_manager;
        let reward_claim_list: Vec<(RewardClaim, RewardClaimDetail)> = self.reward_claim_repo
            .list_all_by_user(db_manager.get_connection().await?.into(), user_id).await?;

        let coin_network_ids: HashSet<Uuid> = reward_claim_list.iter()
            .map(|(claim, _detail)| claim.coin_network_id)
            .collect();
        
        let coin_network_list = self.coin_network_repo
            .list_all_by_ids(db_manager.get_connection().await?.into(), coin_network_ids.into_iter().collect())
            .await?;

        let coin_network_map: HashMap<Uuid, (CoinNetwork, Coin, Network)> = coin_network_list.into_iter()
            .map(|(coin_network, coin, network)| (coin_network.id, (coin_network, coin, network)))
            .collect();

        let combined_responses: Vec<CombinedRewardClaimResponse> = reward_claim_list.into_iter()
            .map(|(claim, detail)| {
                let (coin_network, coin, network) = coin_network_map.get(&claim.coin_network_id).unwrap();
                CombinedRewardClaimResponse::from((claim, detail, coin_network.clone(), coin.clone(), network.clone()))
            })
            .collect();

        Ok(combined_responses)
    }

    async fn create_reward_claim(&self, user_id: Uuid, payload: NewRewardClaimPayload) -> Result<CombinedRewardClaimResponse> {
        let db_manager = &self.db_manager;
        
        let (coin_network, coin, network) = self.coin_network_repo
            .get_with_coin_and_network(
                self.db_manager.get_connection().await?.into(),
                payload.coin_network_id
            )
            .await
            .map_err(|_| Error::CoinNetworkIdNotFound { id: payload.coin_network_id.to_string() })?;

        // todo: payload validation1) user mission 승인된 상태인지 확인 
        // todo: payload validation2) user mission 금액 일치하는지 확인

        // --- user 당 mission 중복 요청 방지 
        if self.reward_claim_repo.get_by_mission_and_user(db_manager.get_connection().await?.into(), payload.mission_id, user_id).await.is_ok() {
            return Err(Error::RewardClaimDuplicate { mission_id: payload.mission_id.to_string(), user_id: user_id.to_string() });
        }

        let scale_factor = BigDecimal::from_str(&format!("1e{}", coin.decimals)).expect("Invalid decimal format");
        let amount_decimal = BigDecimal::from_str(&payload.amount).expect("Invalid amount format");
        let amount_in_smallest_unit = amount_decimal * scale_factor;

        // --- READY
        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            mission_id: payload.mission_id,
            coin_network_id: payload.coin_network_id,
            reward_claim_status: RewardClaimStatus::Ready,
            amount: amount_in_smallest_unit.clone(),
            user_id: user_id,
            user_address: payload.user_address.clone(),
        };

        let reward_claim = self.reward_claim_repo.insert(db_manager.get_connection().await?.into(), new_reward_claim).await?;
        let tx_result_response = match coin.coin_type {
            CoinType::Native => {
                self.process_native_transfer(payload.clone(), amount_in_smallest_unit.clone()).await
            }
            CoinType::FT => {
                self.process_ft_transfer(coin_network.clone(), payload.clone(), amount_in_smallest_unit.clone()).await
            }
            _ => {
                return Err(Error::CoinTypeNotSupported { coin_type: coin.coin_type.to_string() });
            }
        };

        let response = match tx_result_response {
            Ok(response) => response,
            Err(err) => {
                self.reward_claim_repo.update_status(
                    db_manager.get_connection().await?.into(),
                    reward_claim.id,
                    RewardClaimStatus::TransactionFailed
                ).await?;
                return Err(err);
            }
        };

        let reward_claim_status = if response.has_errors {
            RewardClaimStatus::TransactionFailed
        } else {
            RewardClaimStatus::TransactionApproved
        };
        let reward_claim = self.reward_claim_repo.update_status(db_manager.get_connection().await?.into(), reward_claim.id, reward_claim_status).await?;

        let new_reward_claim_detail = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: reward_claim.id,
            transaction_hash: response.transaction_hash.to_string(),
            sended_user_id: user_id,
            sended_user_address: response.receiver_id.to_string(),
        };
        let claim_detail = self.reward_claim_repo.insert_detail(db_manager.get_connection().await?.into(), new_reward_claim_detail).await?;

        if response.has_errors {
            Err(Error::TransactionActionFailed { message: response.error_details.join(", ") })
        }else {
            Ok(CombinedRewardClaimResponse::from((reward_claim, claim_detail, coin_network, coin, network)))
        }
    }

    async fn process_native_transfer(&self, payload: NewRewardClaimPayload, amount_in_smallest_unit: BigDecimal) -> Result<TransactionResultResponse> {
        retry_async(
            || {
                let near_rpc_manager = self.near_rpc_manager.clone();
                let user_address = payload.user_address.clone();
                let amount_in_smallest_unit = amount_in_smallest_unit.clone();
                Box::pin(async move {
                    near_rpc_manager.process_transfer_action(
                        TransferActionType::Native {
                            user_address: user_address.to_string(),
                            amount_in_smallest_unit: amount_in_smallest_unit.clone(),
                        },
                        false
                    ).await
                })
            },
            MAX_RETRY_COUNT,
            RETRY_DELAY
        ).await.map_err(Into::into)
    }
    
    async fn process_ft_transfer(&self, coin_network: CoinNetwork, payload: NewRewardClaimPayload, amount_in_smallest_unit: BigDecimal) -> Result<TransactionResultResponse> {
        let contract_address = coin_network.contract_address.as_ref().ok_or_else(|| Error::InternalServerError { message: "contract_address is empty".to_string() })?;
    
        // todo: storage_deposit check
        let _ = retry_async(
            || {
                let near_rpc_manager = self.near_rpc_manager.clone();
                let contract_address = contract_address.clone();
                let user_address = payload.user_address.clone();
                Box::pin(async move {
                    near_rpc_manager.send_storage_deposit(
                        AccountId::from_str(&contract_address).unwrap(),
                        AccountId::from_str(user_address.as_str()).unwrap(),
                    ).await
                })
            },
            MAX_RETRY_COUNT,
            RETRY_DELAY
        ).await?;
    
        retry_async(
            || {
                let near_rpc_manager = self.near_rpc_manager.clone();
                let contract_address = contract_address.clone();
                let user_address = payload.user_address.clone();
                let amount_in_smallest_unit = amount_in_smallest_unit.clone();
                Box::pin(async move {
                    near_rpc_manager.process_transfer_action(
                        TransferActionType::FtTransfer {
                            ft_contract_id: AccountId::from_str(&contract_address).unwrap(),
                            user_address: user_address.to_string(),
                            amount_in_smallest_unit: amount_in_smallest_unit.clone(),
                        },
                        false
                    ).await
                })
            },
            MAX_RETRY_COUNT,
            RETRY_DELAY
        ).await.map_err(Into::into)
    }
}

