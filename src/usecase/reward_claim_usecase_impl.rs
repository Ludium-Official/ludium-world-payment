use std::{collections::HashMap, sync::Arc};
use async_trait::async_trait;
use uuid::Uuid;
use crate::{
    domain::model::{
        coin::Coin, coin_network::CoinNetwork, network::Network, reward_claim::{
            CombinedRewardClaimResponse, NewRewardClaimPayload, RewardClaim, RewardClaimApprovePayload, RewardClaimApproveResponse, RewardClaimResponse, RewardClaimStatus
        }, reward_claim_detail::NewRewardClaimDetail
    }, port::output::{
        coin_network_repository::CoinNetworkRepository, reward_claim_repository::RewardClaimRepository, rpc_client::RpcClient, DbManager
    }
};
use super::{error::{Error, Result}, utrait::near_usecase::NearUsecase};
use super::utrait::reward_claim_usecase::RewardClaimUsecase;

pub struct RewardClaimUsecaseImpl<D: DbManager, R: RewardClaimRepository, C: CoinNetworkRepository, U: NearUsecase, N: RpcClient> {
    db_manager: Arc<D>,
    reward_claim_repo: Arc<R>,
    coin_network_repo: Arc<C>,
    near_usecase: Arc<U>,
    near_rpc_manager: Arc<N>,
}

impl<D, R, C, U, N> RewardClaimUsecaseImpl<D, R, C, U, N>
where
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
    U: NearUsecase + Send + Sync,
    N: RpcClient + Send + Sync + 'static,
{
    pub fn new(db_manger: Arc<D>, reward_claim_repo: Arc<R>, coin_network_repo: Arc<C>, near_usecase: Arc<U>, near_rpc_manager: Arc<N>) -> Self {
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
impl<D, R, C, U, N> RewardClaimUsecase for RewardClaimUsecaseImpl<D, R, C, U, N>
where 
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
    U: NearUsecase + Send + Sync,
    N: RpcClient + Send + Sync + 'static,
{
    async fn create_reward_claim(&self, payload: NewRewardClaimPayload) -> Result<CombinedRewardClaimResponse> {
        let db_manger = &self.db_manager;
        
        let (coin_network, coin, network) = self.coin_network_repo
            .get_with_coin_and_network(
                self.db_manager.get_connection().await?.into(),
                payload.coin_network_id
            )
            .await
            .map_err(|_| Error::CoinNetworkIdNotFound { id: payload.coin_network_id.to_string() })?;

        if self.reward_claim_repo.get_by_mission_and_user(db_manger.get_connection().await?.into(), payload.mission_id, payload.user_id).await.is_ok() {
            return Err(Error::RewardClaimDuplicate { mission_id: payload.mission_id.to_string(), user_id: payload.user_id.to_string() });
        }

        let reward_claim = self.reward_claim_repo.insert(db_manger.get_connection().await?.into(), payload).await?;
        Ok(CombinedRewardClaimResponse::from((reward_claim, coin_network, coin, network)))
    }

    async fn create_multiple_reward_claims(&self, payloads: Vec<NewRewardClaimPayload>) -> Result<Vec<CombinedRewardClaimResponse>> {
        let db_manager = &self.db_manager;

        let coin_network_ids: Vec<Uuid> = payloads.iter().map(|p| p.coin_network_id).collect();
        let coin_networks = self.coin_network_repo.get_with_coins_and_networks(db_manager.get_connection().await?.into(), coin_network_ids).await?;

        let coin_network_map: HashMap<Uuid, (CoinNetwork, Coin, Network)> = coin_networks.into_iter()
            .map(|(cn, c, n)| (cn.id, (cn, c, n)))
            .collect();

        let mut responses = Vec::new();
        for payload in payloads {
            let (coin_network, coin, network) = coin_network_map.get(&payload.coin_network_id)
                .ok_or_else(|| Error::CoinNetworkIdNotFound { id: payload.coin_network_id.to_string() })?;

            if self.reward_claim_repo.get_by_mission_and_user(db_manager.get_connection().await?.into(), payload.mission_id, payload.user_id).await.is_ok() {
                return Err(Error::RewardClaimDuplicate { mission_id: payload.mission_id.to_string(), user_id: payload.user_id.to_string() });
            }

            let reward_claim = self.reward_claim_repo.insert(db_manager.get_connection().await?.into(), payload).await?;
            // todo : batch insert
            responses.push(CombinedRewardClaimResponse::from((reward_claim, coin_network.clone(), coin.clone(), network.clone())));
        }

        Ok(responses)
    }

    async fn reject_reward_claim(&self, claim_id: Uuid) -> Result<RewardClaimResponse> {
        let db_manager = &self.db_manager;
        
        let reward_claim = self.reward_claim_repo.get(db_manager.get_connection().await?.into(), claim_id).await?;
        if !reward_claim.reward_claim_status.eq(&RewardClaimStatus::Ready) {
            return Err(Error::InvalidClaimStatusForReject);
        }

        let updated_claim = self.reward_claim_repo.update_status(db_manager.get_connection().await?.into(), claim_id, RewardClaimStatus::Rejected).await?;

        Ok(RewardClaimResponse::from(updated_claim))
    }

    async fn approve_reward_claim(&self, user_id: Uuid, claim_id: Uuid, payload: RewardClaimApprovePayload) -> Result<RewardClaimApproveResponse> {
        let db_manager = &self.db_manager;
        let reward_claim = self.reward_claim_repo.get(db_manager.get_connection().await?.into(), claim_id).await?;

        if !reward_claim.reward_claim_status.eq(&RewardClaimStatus::Ready) {
            return Err(Error::InvalidClaimStatusForApprove);
        }

        let new_reward_claim_detail = NewRewardClaimDetail {
            id: Uuid::new_v4(),
            reward_claim_id: claim_id,
            transaction_hash: String::new(),
            sended_user_id: user_id,
            sended_user_address: String::new(),
        };
        
        let near_rpc_manager: Arc<dyn RpcClient> = Arc::clone(&self.near_rpc_manager) as Arc<dyn RpcClient>;
        let tx_result_response = self.near_usecase.relay(&near_rpc_manager, payload.encode_signed_delegate).await;
        match tx_result_response {
            Ok(tx_result) => {
                let updated_claim: RewardClaim;
                let mut detail: NewRewardClaimDetail = new_reward_claim_detail;
                detail.transaction_hash = tx_result.transaction_hash.to_string();
                detail.sended_user_address = tx_result.receiver_id.to_string();

                let reward_claim_status: RewardClaimStatus;
                if tx_result.has_errors {
                    reward_claim_status = RewardClaimStatus::TransactionFailed;
                }else {
                    reward_claim_status = RewardClaimStatus::TransactionApproved;
                }

                updated_claim = self.reward_claim_repo.update_status(
                    db_manager.get_connection().await?.into(),
                    claim_id,
                    reward_claim_status,
                ).await?;
                
                let claim_detail = self.reward_claim_repo.insert_detail(db_manager.get_connection().await?.into(), detail).await?;
                Ok(RewardClaimApproveResponse::from((updated_claim, claim_detail)))
            }
            Err(e) => {
                Err(Error::InternalServerError { message: e.to_string() })
            }
        }
    }
}