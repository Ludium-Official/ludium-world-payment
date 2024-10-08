// #![allow(unused)] // For beginning only.

mod adapter;
mod port;
mod domain;
mod config;
mod usecase;
mod state; 

use std::net::IpAddr;
use std::str::FromStr;
use std::{net::SocketAddr, path::PathBuf};
use std::sync::Arc;
use adapter::input::web::middleware::permission;
use axum::{middleware, Router};
use axum_server::tls_rustls::RustlsConfig;
use config::log::request_logging_middleware;
use config::swagger::ApiDoc;
use state::AppState;
use tower_cookies::CookieManagerLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::{
    adapter::input::{
        routes_static, 
        web::{self, middleware::{auth, response}, routes_hello}
    }, 
    config::config
};
pub use self::adapter::input::error::Result;

#[tokio::main]
async fn main() -> Result<()>{    
    let config = config().await;
    let app_state = Arc::new(AppState::new(&config).await?);
    
    let mut routes_all = Router::new()
        .merge(routes_hello::routes());
    let routes_auth_apis = web::routes_network::routes(Arc::clone(&app_state))
        .merge(web::routes_reward_claim::routes(Arc::clone(&app_state)))
        .merge(web::routes_coin::routes(Arc::clone(&app_state)))
        .merge(web::routes_coin_network::routes(Arc::clone(&app_state)))
        .route_layer(middleware::from_fn(permission::mw_require_auth));

    if config.is_local() {
        tracing::info!("dev routes enabled");
        routes_all = routes_all.merge(web::_dev_routes_login::routes());
    }

    if config.is_development() {
        tracing::info!("swagger-ui enabled");
        routes_all = routes_all.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
    }

    routes_all = routes_all
        .nest("/api", routes_auth_apis)
        .layer(middleware::map_response(response::mapper))
        .layer(request_logging_middleware())
        .layer(middleware::from_fn_with_state(
            Arc::clone(&app_state),
            auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());


    let ip_addr = IpAddr::from_str(config.server_host()).unwrap();
    let addr = SocketAddr::from((ip_addr, config.server_port()));
    tracing::info!("listening on {}", addr);

    if config.is_local() || !config.server_use_tls() {
        axum_server::bind(addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    } else {
        let tls_config = RustlsConfig::from_pem_file(
            PathBuf::from("./self_signed_certs")
                .join("cert.pem"),
            PathBuf::from("./self_signed_certs")
                .join("key.pem"),
        )
        .await
        .unwrap();

        tracing::info!("TLS file loaded");

        axum_server::bind_rustls(addr, tls_config)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    }

    Ok(())
}