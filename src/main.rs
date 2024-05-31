mod adapter;
mod port;
mod domain;
mod config;
mod usecase;
mod state; 

use std::sync::Arc;
use axum::{middleware, Router};
use config::log::request_logging_middleware;
use state::AppState;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use crate::{
    adapter::input::{
        routes_static, 
        web::{self, middleware::{auth, response}, routes_hello, _dev_routes_login}
    }, 
    config::{config, log::{self}},
};
pub use self::adapter::input::error::Result;

#[tokio::main]
async fn main() -> Result<()>{
    log::init_tracing();
    let config = config().await;
    let app_state = Arc::new(AppState::new(&config).await?);

    let mut routes_apis: Router;
    let mut routes_all: Router = Router::new();

    if config.is_dev() {
        tracing::debug!("Running in dev mode");
        routes_apis = web::_dev_routes_user::routes(Arc::clone(&app_state));
        routes_all = routes_all.merge(web::_dev_routes_login::routes());
    }

    routes_apis = web::routes_coin::routes(Arc::clone(&app_state))
        .merge(web::routes_network::routes(Arc::clone(&app_state)))
        .merge(web::routes_reward_claim::routes(Arc::clone(&app_state)))
        .route_layer(middleware::from_fn(auth::mw_require_auth));

    routes_all = routes_all
        .merge(routes_hello::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(response::mapper))
        .layer(request_logging_middleware())
        .layer(middleware::from_fn_with_state(
            Arc::clone(&app_state),
            auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = TcpListener::bind(format!("{}:{}", config.server_host(), config.server_port())).await.unwrap();
    tracing::info!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}