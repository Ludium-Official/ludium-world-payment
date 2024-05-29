#![allow(unused)] // For beginning only.

mod adapter;
mod port;
mod domain;
mod config;
mod usecase;
mod state; 

use std::sync::Arc;
use adapter::{input::ctx::Ctx, output::persistence::db::postgres::{user_repository_impl::PostgresUserRepository, PostgresDbManager}};
use axum::{middleware, Extension, Router};
use state::AppState;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use usecase::near_usecase_impl::NearUsecaseImpl;
use crate::{
    adapter::{
        input::{
            routes_static, 
            web::{self, middleware::{auth, response}, routes_hello, _dev_routes_login}
        }, 
        output::{
            near::rpc_client::NearRpcManager, 
            persistence::db::postgres::{
                coin_network_repository_impl::PostgresCoinNetworkRepository, 
                coin_repository_impl::PostgresCoinRepository, 
                network_repository_impl::PostgresNetworkRepository, 
                reward_claim_repository_impl::PostgresRewardClaimRepository
            }
        }
    }, 
    config::{config, log::{self}}, usecase::{reward_claim_usecase_impl::RewardClaimUsecaseImpl, utrait::reward_claim_usecase::RewardClaimUsecase}
};
pub use self::adapter::input::error::Result;

use near_fetch::signer::KeyRotatingSigner;
use ::config::{Config, File as ConfigFile};
use once_cell::sync::Lazy;
use near_crypto::InMemorySigner;

#[tokio::main]
async fn main() -> Result<()>{
    log::init_tracing();
    let config = config().await;
    let app_state = Arc::new(AppState::new(&config).await?);

    let routes_apis = web::_dev_routes_user::routes(Arc::clone(&app_state))
        .merge(web::routes_coin::routes(Arc::clone(&app_state)))
        .merge(web::routes_network::routes(Arc::clone(&app_state)))
        .merge(web::routes_reward_claim::routes(Arc::clone(&app_state)))
        .route_layer(middleware::from_fn(auth::mw_require_auth));
    
    // TODO: Add a middleware to resolve the context real value
    let ctx = Ctx::new(0);

    let routes_all: Router = Router::new()
        .merge(routes_hello::routes())
        .merge(_dev_routes_login::routes())
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