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
    adapter::input::{
            error::Error, 
            routes_static, 
            web::{self, middleware::{auth, response}, routes_hello, routes_login}
        }, 
    config::{config, log::{self, log_request}}
};
pub use self::adapter::input::error::Result;

use near_fetch::signer::KeyRotatingSigner;
use ::config::{Config, File as ConfigFile};
use once_cell::sync::Lazy;
use near_crypto::InMemorySigner;

// load config from toml and setup jsonrpc client
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


#[derive(Clone)]
struct AppState {
    db_manager: PostgresDbManager,
    user_repo: PostgresUserRepository,
    near_rpc_client: Arc<near_fetch::Client>
}

#[tokio::main]
async fn main() -> Result<()>{
    log::init_tracing();
    let config = config().await;

    println!("Hello, world!, {}", config.db_url().to_string());

    let db_manager = PostgresDbManager::new(&config.db_url()).await?;
    let user_repo = PostgresUserRepository;
    let near_rpc_client = Arc::new(config.near_network_config.rpc_client());

    let app_state = Arc::new(AppState {
        db_manager: db_manager,
        user_repo,
        near_rpc_client
    });

    let routes_apis = web::routes_user::routes(Arc::clone(&app_state))
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