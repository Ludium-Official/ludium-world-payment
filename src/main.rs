#![allow(unused)] // For beginning only.

mod adapter;
mod port;
mod domain;
mod config;
mod usecase;
use std::sync::Arc;

use adapter::{input::ctx::Ctx, output::persistence::db::postgres::{user_repository_impl::PostgresUserRepository, PostgresDbManager}};
use axum::{http::{Method, Uri}, middleware, response::{IntoResponse, Response}, Extension, Json, Router};
use serde_json::json;
use tokio::{net::TcpListener, sync::RwLock};
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;
use crate::{
    adapter::{input::{
            error::Error, 
            routes_static, 
            web::{self, middleware::{auth, response}, routes_hello, routes_login}
        }, output::{near::NearRpcManager, persistence::db::postgres::{coin_network_repository_impl::PostgresCoinNetworkRepository, coin_repository_impl::PostgresCoinRepository, network_repository_impl::PostgresNetworkRepository, reward_claim_repository_impl::PostgresRewardClaimRepository}}}, 
    config::{config, log::{self, log_request}}, usecase::{reward_claim_usecase_impl::RewardClaimUsecaseImpl, utrait::reward_claim_usecase::RewardClaimUsecase}
};
pub use self::adapter::input::error::Result;

use near_fetch::signer::KeyRotatingSigner;
use ::config::{Config, File as ConfigFile};
use once_cell::sync::Lazy;
use near_crypto::InMemorySigner;

// region: --- near signer setup
static LOCAL_CONF: Lazy<Config> = Lazy::new(|| {
    Config::builder()
        .add_source(ConfigFile::with_name("config.toml"))
        .build()
        .unwrap()
});

static ROTATING_SIGNER: Lazy<KeyRotatingSigner> = Lazy::new(|| {
    let path = LOCAL_CONF
        .get::<String>("keys_filename")
        .expect("Failed to read 'keys_filename' from config");
    let keys_file = std::fs::File::open(path).expect("Failed to open keys file");
    let signers: Vec<InMemorySigner> =
        serde_json::from_reader(keys_file).expect("Failed to parse keys file");

    KeyRotatingSigner::from_signers(signers)
});
// endregion

#[derive(Clone)]
struct AppState {
    db_manager: Arc<PostgresDbManager>,
    user_repo: Arc<PostgresUserRepository>,
    coin_repo: Arc<PostgresCoinRepository>,
    network_repo: Arc<PostgresNetworkRepository>,
    coin_network_repo: Arc<PostgresCoinNetworkRepository>,
    reward_claim_repo: Arc<PostgresRewardClaimRepository>,
    reward_claim_usecase: Arc<dyn RewardClaimUsecase + Send + Sync>,
    near_rpc_manager: Arc<NearRpcManager>, // todo: delete me! process_meta_tx use me 
}

#[tokio::main]
async fn main() -> Result<()>{
    log::init_tracing();
    let config = config().await;

    println!("Hello, world!, {}", config.db_url().to_string());

    let db_manager = Arc::new(PostgresDbManager::new(&config.db_url()).await?);
    let user_repo = Arc::new(PostgresUserRepository);
    let coin_repo = Arc::new(PostgresCoinRepository);
    let network_repo = Arc::new(PostgresNetworkRepository);
    let coin_network_repo = Arc::new(PostgresCoinNetworkRepository);
    let reward_claim_repo = Arc::new(PostgresRewardClaimRepository);
    let near_rpc_manager = Arc::new(NearRpcManager::new(config.near_network_config.rpc_client()));
    let reward_claim_usecase: Arc<dyn RewardClaimUsecase + Send + Sync> = Arc::new(RewardClaimUsecaseImpl::new(
        Arc::clone(&db_manager),
        Arc::clone(&reward_claim_repo),
        Arc::clone(&coin_network_repo),
        Arc::clone(&near_rpc_manager),
    ));

    let app_state = Arc::new(AppState {
        db_manager: Arc::clone(&db_manager),
        user_repo: Arc::clone(&user_repo),
        coin_repo: Arc::clone(&coin_repo),
        network_repo: Arc::clone(&network_repo),
        coin_network_repo: Arc::clone(&coin_network_repo),
        reward_claim_repo: Arc::clone(&reward_claim_repo),
        reward_claim_usecase: Arc::clone(&reward_claim_usecase),
        near_rpc_manager: Arc::clone(&near_rpc_manager),
    });

    let routes_apis = web::routes_user::routes(Arc::clone(&app_state))
        .merge(web::routes_coin::routes(Arc::clone(&app_state)))
        .merge(web::routes_network::routes(Arc::clone(&app_state)))
        .merge(web::routes_reward_claim::routes(Arc::clone(&app_state)))
        .route_layer(middleware::from_fn(auth::mw_require_auth));
    
    // TODO: Add a middleware to resolve the context
    let ctx = Ctx::new(0);

    let routes_all: Router = Router::new()
        .merge(routes_hello::routes())
        .merge(routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(response::mapper))
        .layer(middleware::from_fn_with_state(
            Arc::clone(&app_state),
            auth::mw_ctx_resolver,
        ))
        .layer(Extension(ctx)) 
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind(format!("{}:{}", config.server_host(), config.server_port())).await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}