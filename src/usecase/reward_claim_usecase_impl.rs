use std::{collections::{HashMap, HashSet}, sync::Arc};
use async_trait::async_trait;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use crate::{
    adapter::output::near::{retry_async, rpc_client::NearRpcManager, MAX_RETRY_COUNT, RETRY_DELAY}, domain::model::{
        coin::{Coin, CoinType}, coin_network::CoinNetwork, near::{TransactionResultResponse, TransferActionType}, network::Network, reward_claim::{
            CombinedRewardClaimResponse, NewRewardClaim, NewRewardClaimPayload, ResourceType, RewardClaim, RewardClaimStatus
        }, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}
    }, port::output::{
        coin_network_repository::CoinNetworkRepository, detailed_posting_repository::DetailedPostingRepository, mission_submit_repository::MissionSubmitRepository, reward_claim_repository::RewardClaimRepository, rpc_client::RpcClient, DbManager, UserRepository
    }
};
use super::error::{Error, Result};
use super::utrait::reward_claim_usecase::RewardClaimUsecase;
use std::str::FromStr;
use near_primitives::types::AccountId;

pub struct RewardClaimUsecaseImpl<D: DbManager, R: RewardClaimRepository, C: CoinNetworkRepository, U: UserRepository, MS: MissionSubmitRepository, DP: DetailedPostingRepository> {
    db_manager: Arc<D>,
    reward_claim_repo: Arc<R>,
    coin_network_repo: Arc<C>,
    near_rpc_manager: Arc<NearRpcManager>,
    user_repo: Arc<U>,
    mission_submit_repo: Arc<MS>,
    detailed_posting_repo: Arc<DP>,
}

impl<D, R, C, U, MS, DP> RewardClaimUsecaseImpl<D, R, C, U, MS, DP>
where
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    MS: MissionSubmitRepository + Send + Sync,
    DP: DetailedPostingRepository + Send + Sync,
{
    pub fn new(db_manger: Arc<D>, reward_claim_repo: Arc<R>, coin_network_repo: Arc<C>, near_rpc_manager: Arc<NearRpcManager>, user_repo: Arc<U>, mission_submit_repo: Arc<MS>, detailed_posting_repo: Arc<DP>) -> Self {
        Self {
            db_manager: db_manger,
            reward_claim_repo,
            coin_network_repo,
            near_rpc_manager,
            user_repo,
            mission_submit_repo,
            detailed_posting_repo,
        }
    }

    async fn validate_user(&self, user_id: Uuid) -> Result<()> {
        self.user_repo.get(self.db_manager.get_connection().await?.into(), user_id).await.map_err(|_| {
            tracing::error!("User Not Found: {}", user_id.to_string());
            Error::UserIdNotFound
        })?;
        Ok(())
    }

    async fn validate_resource(&self, resource_type_str: &str, user_id: Uuid, resource_id: Uuid) -> Result<ResourceType> {
        let resource_type = match resource_type_str.to_uppercase().as_str() {
            "MISSION" => ResourceType::Mission,
            "DETAILED_POSTING" => ResourceType::DetailedPosting,
            _ => return Err(Error::InvalidResourceType{ message: format!("Invalid resource_type: {}", resource_type_str) }),
        };

        if resource_type == ResourceType::Mission {
            // --- mission_submit validation
            let mission_submit = self.mission_submit_repo.get(self.db_manager.get_connection().await?.into(), user_id, resource_id).await.map_err(|_| {
                tracing::error!("Mission Submit Not Found: {}", resource_id.to_string());
                Error::MissionSubmitIdNotFound
            })?;
            if !mission_submit.is_approved() {
                tracing::error!("Mission Submit Not Approved: {}", resource_id.to_string());
                return Err(Error::MissionSubmitNotApproved);
            }
        } else {
            // --- detailed_posting validation
            let detailed_posting = self.detailed_posting_repo.get(self.db_manager.get_connection().await?.into(), resource_id).await.map_err(|_| {
                tracing::error!("Detailed Posting Not Found: {}", resource_id.to_string());
                Error::DetailedPostingIdNotFound
            })?;
            if !detailed_posting.is_approved() {
                tracing::error!("Detailed Posting Not Approved: {}", resource_id.to_string());
                return Err(Error::DetailedPostingNotApproved);
            }
        }
        Ok(resource_type)
    }

    async fn handle_existing_reward_claim(&self, existed_reward_claim: RewardClaim) -> Result<()> {
        match existed_reward_claim.reward_claim_status {
            // --- user 중복 요청 방지 (only READY, TRANSACTION_APPROVED)
            RewardClaimStatus::Ready => {
                tracing::error!("[Ready] Reward Claim Duplicate: Resource Id: {}, Resource Type: {}, User Id: {}", existed_reward_claim.resource_id, existed_reward_claim.resource_type, existed_reward_claim.user_id);
                return Err(Error::RewardClaimDuplicate);
            }
            RewardClaimStatus::TransactionApproved => {
                tracing::error!("[TransactionApproved] Reward Claim Duplicate: Resource Id: {}, Resource Type: {}, User Id: {}", existed_reward_claim.resource_id, existed_reward_claim.resource_type, existed_reward_claim.user_id);
                return Err(Error::RewardClaimDuplicate);
            }
            RewardClaimStatus::TransactionFailed => {
                // --- 실패한 트랜잭션 재시도 (TRANSACTION_FAILED -> READY)
                tracing::debug!(
                    "[Retry][TransactionFailed -> Ready] Reward Claim Transaction Failed: Resource Id: {}, Resource Type: {}, User Id: {}. Retrying...",
                    existed_reward_claim.resource_id,
                    existed_reward_claim.resource_type,
                    existed_reward_claim.user_id
                );
                self.reward_claim_repo.update_status(
                    self.db_manager.get_connection().await?.into(),
                    existed_reward_claim.id,
                    RewardClaimStatus::Ready,
                    true
                ).await?;
            }
        }
        Ok(())
    }

    async fn create_new_reward_claim(&self, payload: &NewRewardClaimPayload, resource_type: ResourceType, amount_in_smallest_unit: BigDecimal, user_id: Uuid) -> Result<RewardClaim> {
        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            resource_id: payload.resource_id,
            resource_type: resource_type.clone(),
            coin_network_id: payload.coin_network_id,
            reward_claim_status: RewardClaimStatus::Ready,
            amount: amount_in_smallest_unit.clone(),
            user_id,
            user_address: payload.user_address.clone(),
        };
        
        let reward_claim = self.reward_claim_repo.insert(self.db_manager.get_connection().await?.into(), new_reward_claim).await?;
        Ok(reward_claim)
    }

}

#[async_trait]
impl<D, R, C, U, MS, DP> RewardClaimUsecase for RewardClaimUsecaseImpl<D, R, C, U, MS, DP>
where 
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
    U: UserRepository + Send + Sync,
    MS: MissionSubmitRepository + Send + Sync,
    DP: DetailedPostingRepository + Send + Sync,
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
    
        // --- user validation
        self.validate_user(user_id).await?;

        // --- resource validation
        let resource_type = self.validate_resource(&payload.resource_type, user_id, payload.resource_id).await?;

        // todo: payload validation2) user mission 금액 일치하는지 확인
        let (coin_network, coin, network) = self.coin_network_repo
            .get_with_coin_and_network(
                self.db_manager.get_connection().await?.into(),
                payload.coin_network_id
            )
            .await
            .map_err(|_| {
                tracing::error!("Coin Network Id Not Found: {}", payload.coin_network_id.to_string());
                Error::CoinNetworkIdNotFound
            })?;


        let scale_factor = BigDecimal::from_str(&format!("1e{}", coin.decimals)).expect("Invalid decimal format");
        let amount_decimal: BigDecimal = BigDecimal::from_str(&payload.amount).expect("Invalid amount format");
        let amount_in_smallest_unit = amount_decimal * scale_factor;
        
        let existed_reward_claim_result = self.reward_claim_repo.get_by_resource_and_user(db_manager.get_connection().await?.into(), resource_type.clone(), payload.resource_id, user_id).await;
        let reward_claim = match existed_reward_claim_result {
            Ok(existed_reward_claim) => {
                self.handle_existing_reward_claim(existed_reward_claim.clone()).await?;
                existed_reward_claim
            }
            Err(_) => { 
                self.create_new_reward_claim(&payload, resource_type, amount_in_smallest_unit.clone(), user_id).await?
            }
        };

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
                    RewardClaimStatus::TransactionFailed,
                    false
                ).await?;
                return Err(err);
            }
        };

        let reward_claim_status = if response.has_errors {
            RewardClaimStatus::TransactionFailed
        } else {
            RewardClaimStatus::TransactionApproved
        };
        let reward_claim = self.reward_claim_repo.update_status(db_manager.get_connection().await?.into(), reward_claim.id, reward_claim_status, false).await?;

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
