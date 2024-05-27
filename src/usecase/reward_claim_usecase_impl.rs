use std::{collections::HashMap, sync::Arc};
use async_trait::async_trait;
use uuid::Uuid;
use crate::{domain::model::{coin::Coin, coin_network::CoinNetwork, network::Network, reward_claim::{NewRewardClaimPayload, RewardClaim, RewardClaimResponse, RewardClaimStatus}}, port::output::{coin_network_repository::CoinNetworkRepository, reward_claim_repository::RewardClaimRepository, DbManager}};
use crate::adapter::input::error::{Result, Error};
use super::utrait::reward_claim_usecase::RewardClaimUsecase;

pub struct RewardClaimUsecaseImpl<D: DbManager, R: RewardClaimRepository, C: CoinNetworkRepository> {
    db_manager: Arc<D>,
    reward_claim_repo: Arc<R>,
    coin_network_repo: Arc<C>,
}

impl<D, R, C> RewardClaimUsecaseImpl<D, R, C>
where
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
{
    pub fn new(db_manger: Arc<D>, reward_claim_repo: Arc<R>, coin_network_repo: Arc<C>) -> Self {
        Self {
            db_manager: db_manger,
            reward_claim_repo,
            coin_network_repo,
        }
    }
}

#[async_trait]
impl<D, R, C> RewardClaimUsecase for RewardClaimUsecaseImpl<D, R, C>
where 
    D: DbManager + Send + Sync,
    R: RewardClaimRepository + Send + Sync,
    C: CoinNetworkRepository + Send + Sync,
{
    async fn create_reward_claim(&self, payload: NewRewardClaimPayload) -> Result<RewardClaimResponse> {
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
        Ok(RewardClaimResponse::from((reward_claim, coin_network, coin, network)))
    }

    async fn create_multiple_reward_claims(&self, payloads: Vec<NewRewardClaimPayload>) -> Result<Vec<RewardClaimResponse>> {
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
            responses.push(RewardClaimResponse::from((reward_claim, coin_network.clone(), coin.clone(), network.clone())));
        }

        Ok(responses)

    }
}