use std::{collections::{HashMap, HashSet}, sync::Arc, time::Duration};
use async_trait::async_trait;
use bigdecimal::{ToPrimitive, BigDecimal};
use tokio::time::sleep;
use uuid::Uuid;
use crate::{
    adapter::output::near::rpc_client::NearRpcManager, domain::model::{
        coin::{Coin, CoinType}, coin_network::CoinNetwork, near::{TransactionResultResponse, TransferActionType}, network::Network, reward_claim::{
            CombinedRewardClaimResponse, NewRewardClaim, NewRewardClaimPayload, RewardClaim, RewardClaimStatus
        }, reward_claim_detail::{NewRewardClaimDetail, RewardClaimDetail}
    }, port::output::{
        coin_network_repository::CoinNetworkRepository, reward_claim_repository::RewardClaimRepository, rpc_client::RpcClient, DbManager
    }
};
use super::{error::{Error, Result}, utrait::near_usecase::NearUsecase};
use super::utrait::reward_claim_usecase::RewardClaimUsecase;
use std::str::FromStr;
use near_primitives::types::AccountId;

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

    // actually, at this time don't need to meta_tx logic, just send_tx is good. but for the future, I will keep this relayer code
    async fn create_reward_claim(&self, user_id: Uuid, payload: NewRewardClaimPayload) -> Result<CombinedRewardClaimResponse> {
        tracing::debug!("create_reward_claim {:}", user_id);
        let db_manager = &self.db_manager;
        
        let (coin_network, coin, network) = self.coin_network_repo
            .get_with_coin_and_network(
                self.db_manager.get_connection().await?.into(),
                payload.coin_network_id
            )
            .await
            .map_err(|_| Error::CoinNetworkIdNotFound { id: payload.coin_network_id.to_string() })?;

        // --- user 당 mission 중복 요청 방지 
        if self.reward_claim_repo.get_by_mission_and_user(db_manager.get_connection().await?.into(), payload.mission_id, payload.user_id).await.is_ok() {
            return Err(Error::RewardClaimDuplicate { mission_id: payload.mission_id.to_string(), user_id: payload.user_id.to_string() });
        }

        let scale_factor = BigDecimal::from_str(&format!("1e{}", coin.decimals)).expect("Invalid decimal format");
        let amount_decimal = BigDecimal::from_str(&payload.amount).expect("Invalid amount format");
        let amount_in_smallest_unit = (amount_decimal * scale_factor).to_u128().ok_or(Error::InvalidAmountConversion)?;

        // --- READY
        let new_reward_claim = NewRewardClaim {
            id: Uuid::new_v4(),
            mission_id: payload.mission_id,
            coin_network_id: payload.coin_network_id,
            reward_claim_status: RewardClaimStatus::Ready,
            amount: amount_in_smallest_unit as i64,
            user_id: payload.user_id,
            user_address: payload.user_address.clone(),
        };

        let reward_claim = self.reward_claim_repo.insert(db_manager.get_connection().await?.into(), new_reward_claim).await?;

        // --- user 당 coin_network 이미 처리중인 PENDING_APPROVAL 상태 확인 및 스핀락 로직 (5회 시도(10s) 후 실패시 에러 반환)
        let mut attempts = 0;
        while self.reward_claim_repo.has_pending_approval(db_manager.get_connection().await?.into(), payload.user_id, payload.coin_network_id).await? {
            tracing::debug!("transcation waiting... - user_id: {}, coin_network_id: {}", payload.user_id, payload.coin_network_id); 
            if attempts >= 5 {
                return Err(Error::TranscationTimeoutFailed { message: format!("wating transcation timeout failed - user_id: {}, coin_network_id: {}", payload.user_id, payload.coin_network_id)});
            }
            attempts += 1;
            sleep(Duration::from_secs(5)).await;
        }

        // 1. reward_claims 에서 해당 user에게 해당 contract 전송 여부 확인 
                // 1-1. 있다면 그냥 바로 ft_transfer
                // 1-2. 없다면, storage_deposit_of로 해당 user가 해당 contract 잔액 조회했는지 확인 
                    // 1-2-1. 이미 deposit 했으면 ft_transfer
                    // 1-2-2. 없다면 deposit 후 ft_transfer
        let tx_result_response: Result<TransactionResultResponse>;
        match coin.coin_type {
            CoinType::Native => {
                let signed_delegate_action = self.near_rpc_manager.create_transfer_signed_delegate_action(
                    TransferActionType::Native {
                        user_address: payload.user_address.to_string(),
                        amount_in_smallest_unit,
                    }
                ).await?;

                if !signed_delegate_action.verify() {
                    return Err(Error::TranscationActionVerifyFailed)
                }

                // --- PENDING_APPROVAL
                self.reward_claim_repo.update_status(
                    db_manager.get_connection().await?.into(),
                    reward_claim.id,
                    RewardClaimStatus::PendingApproval
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
                // println!("deposit_result: {:?}", deposit_result);
                    
                let signed_delegate_action = self.near_rpc_manager.create_transfer_signed_delegate_action(
                    TransferActionType::FtTransfer {
                        ft_contract_id: AccountId::from_str(contract_address).unwrap(),
                        user_address: payload.user_address.to_string(),
                        amount_in_smallest_unit,
                    }
                ).await?;
                if !signed_delegate_action.verify() {
                    return Err(Error::TranscationActionVerifyFailed)
                }

                // --- PENDING_APPROVAL
                self.reward_claim_repo.update_status(
                    db_manager.get_connection().await?.into(),
                    reward_claim.id,
                    RewardClaimStatus::PendingApproval
                ).await?;

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

                // --- TRANSACTION_APPROVED or TRANSACTION_FAILED
                let reward_claim = self.reward_claim_repo.update_status(db_manager.get_connection().await?.into(), reward_claim.id, reward_claim_status).await?;
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

