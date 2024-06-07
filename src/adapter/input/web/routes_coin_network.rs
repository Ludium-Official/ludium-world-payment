use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Router, Json, routing::get};
use serde::Deserialize;
use utoipa::IntoParams;
use crate::adapter::input::ctx::Ctx;
use crate::domain::model::coin_network::CoinNetworkDetailsResponse;
use crate::port::output::coin_network_repository::CoinNetworkRepository;
use crate::port::output::DbManager;
use crate::AppState;
use crate::adapter::input::error::Result;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/coin-networks",  get(list_coin_networks))
        .with_state(state)
}

#[derive(Deserialize, IntoParams)]
pub struct NetworkCodeQuery {
    network_code: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/coin-networks",
    params(NetworkCodeQuery),
    responses(
        (status = 200, description = "List of coin networks", body = Vec<CoinNetworkDetailsResponse>),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 500, description = "Internel Server Error", body = ErrorResponse)
    ),
    tag = "CoinNetwork"
)]
pub async fn list_coin_networks(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Query(query): Query<NetworkCodeQuery>,
) -> Result<Json<Vec<CoinNetworkDetailsResponse>>> {
    let coin_network_details = if let Some(network_code) = query.network_code {
        state
            .coin_network_repo
            .list_all_by_network_code(state.db_manager.get_connection().await?, network_code)
            .await?
    } else {
        state
            .coin_network_repo
            .list_all(state.db_manager.get_connection().await?)
            .await?
    };

    Ok(Json(coin_network_details.into_iter().map(CoinNetworkDetailsResponse::from).collect()))
}
