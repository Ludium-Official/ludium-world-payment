#![allow(unused)] // For beginning only.

mod adapter;
mod port;
mod domain;
mod config;

use std::sync::Arc;

use adapter::{input::ctx::Ctx, output::persistence::db::postgres::{user_repository_impl::PostgresUserRepository, PostgresDbManager}};
use axum::{http::{Method, Uri}, middleware, response::{IntoResponse, Response}, Extension, Json, Router};
use serde_json::json;
use tokio::{net::TcpListener, sync::RwLock};
use tower_cookies::CookieManagerLayer;
use uuid::Uuid;
use crate::{
    adapter::{
        input::{
            error::Error, 
            response, 
            routes_static, 
            web::{self, middleware::auth, routes_hello, routes_login}
        }
        }, 
    config::{config, log::{self, log_request}}
};
pub use self::adapter::input::error::Result;

// TODO: Improve this
// pub type SharedState = Arc<AppState>;

#[derive(Clone)]
struct AppState {
    db_manager: PostgresDbManager,
    user_repo: PostgresUserRepository,
}

#[tokio::main]
async fn main() -> Result<()>{
    log::init_tracing();
    // let state = SharedState::default();
    let config = config().await;

    println!("Hello, world!, {}", config.db_url().to_string());

    let db_manager = PostgresDbManager::new(&config.db_url()).await?;
    let user_repo = PostgresUserRepository;

    let app_state = AppState {
        db_manager: db_manager,
        user_repo,
    };

    let routes_apis = web::routes_user::routes(app_state.clone())
        .route_layer(middleware::from_fn(auth::mw_require_auth));
    
    // TODO: Add a middleware to resolve the context
    let ctx = Ctx::new(0);

    let routes_all: Router = Router::new()
        .merge(routes_hello::routes())
        .merge(routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(response::mapper))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
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