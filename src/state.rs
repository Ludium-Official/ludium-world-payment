use std::sync::Arc;
use crate::adapter::output::persistence::db::postgres::{PostgresDbManager, user_repository_impl::PostgresUserRepository};
use crate::config::Config;
use crate::usecase::{reward_claim_usecase_impl::RewardClaimUsecaseImpl, utrait::reward_claim_usecase::RewardClaimUsecase};
use crate::adapter::output::near::rpc_client::NearRpcManager;
use crate::adapter::output::persistence::db::postgres::{
    coin_network_repository_impl::PostgresCoinNetworkRepository,
    coin_repository_impl::PostgresCoinRepository,
    network_repository_impl::PostgresNetworkRepository,
    reward_claim_repository_impl::PostgresRewardClaimRepository
};
use crate::adapter::input::error::Result;
use crate::port::output::db_manager::DbManager;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db_manager: Arc<PostgresDbManager>,
    pub user_repo: Arc<PostgresUserRepository>,
    pub coin_repo: Arc<PostgresCoinRepository>,
    pub network_repo: Arc<PostgresNetworkRepository>,
    pub coin_network_repo: Arc<PostgresCoinNetworkRepository>,
    pub reward_claim_repo: Arc<PostgresRewardClaimRepository>,
    pub reward_claim_usecase: Arc<dyn RewardClaimUsecase + Send + Sync>,
    pub near_rpc_manager: Arc<NearRpcManager>, 
}

impl AppState {
    pub async fn new(config: &Config) -> Result<Self> {
        let db_manager = Arc::new(PostgresDbManager::new(&config.db_url()).await?);

        {
            let _ = db_manager.get_connection().await?;
            tracing::info!("established postgres db connection");
        }

        
        let user_repo = Arc::new(PostgresUserRepository);
        let coin_repo = Arc::new(PostgresCoinRepository);
        let network_repo = Arc::new(PostgresNetworkRepository);
        let coin_network_repo = Arc::new(PostgresCoinNetworkRepository);
        let reward_claim_repo = Arc::new(PostgresRewardClaimRepository);
        let near_rpc_manager = Arc::new(NearRpcManager::new(
            config.near_network_config().rpc_client(),
            config.signer().clone(),
            config.near_network_config().whitelisted_contracts.clone(),
            config.near_network_config().whitelisted_senders.clone(),
        ));
        let reward_claim_usecase: Arc<dyn RewardClaimUsecase + Send + Sync> = Arc::new(RewardClaimUsecaseImpl::new(
            Arc::clone(&db_manager),
            Arc::clone(&reward_claim_repo),
            Arc::clone(&coin_network_repo),
            Arc::clone(&near_rpc_manager),
        ));

        Ok(Self {
            config: config.clone(),
            db_manager,
            user_repo,
            coin_repo,
            network_repo,
            coin_network_repo,
            reward_claim_repo,
            reward_claim_usecase,
            near_rpc_manager,
        })
    }
}
