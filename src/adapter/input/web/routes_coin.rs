use std::sync::Arc;
use axum::extract::{Query, State};
use axum::{Router, Json, extract::Path, routing::get};
use serde::Deserialize;
use crate::adapter::input::ctx::Ctx;
use crate::domain::model::coin::{CoinResponse, NewCoinPayload};
use crate::domain::model::coin_network::CoinNetworkResponse;
use crate::port::output::coin_network_repository::CoinNetworkRepository;
use crate::port::output::coin_repository::CoinRepository;
use crate::port::output::DbManager;
use crate::AppState;
use crate::adapter::input::error::{Error, Result};
use uuid::Uuid; 

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/coins",  get(list_coins))
        .route("/coins/:id", get(get_coin))
        .route("/coins/:id/networks", get(get_coin_networks_by_coin_id))
        .with_state(state)
}

#[allow(unused)]
async fn create_coin(
	State(state): State<Arc<AppState>>,
    _ctx: Ctx,
	Json(new_coin_payload): Json<NewCoinPayload>,
) -> Result<Json<CoinResponse>> {
	let coin = state.coin_repo.insert(
        state.db_manager.get_connection().await?,
        new_coin_payload,
    ).await?;

	Ok(Json(CoinResponse::from(coin)))
}

#[derive(Deserialize)]
struct NetworkCodeQuery {
    network_code: Option<String>,
}

async fn list_coins(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Query(query): Query<NetworkCodeQuery>,
) -> Result<Json<Vec<CoinResponse>>> {
    let coins: Vec<CoinResponse> = if let Some(network_code) = query.network_code {
        state
            .coin_repo
            .list_by_network_code(state.db_manager.get_connection().await?, network_code)
            .await?
            .into_iter()
            .map(CoinResponse::from)
            .collect()
    } else {
        state
            .coin_repo
            .list(state.db_manager.get_connection().await?)
            .await?
            .into_iter()
            .map(CoinResponse::from)
            .collect()
    };

    Ok(Json(coins))
}

async fn get_coin(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Path(id): Path<String>
) -> Result<Json<CoinResponse>> {
    let id = Uuid::parse_str(&id).map_err(|_| Error::UUIDParsingError{ message: format!("Invalid UUID: {}", id)})?;
    let coin = state
        .coin_repo
        .get(state.db_manager.get_connection().await?, id)
        .await?;

    Ok(Json(CoinResponse::from(coin)))
}


async fn get_coin_networks_by_coin_id(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Path(id): Path<String>
) -> Result<Json<Vec<CoinNetworkResponse>>> {
    let id = Uuid::parse_str(&id).map_err(|_| Error::UUIDParsingError{ message: format!("Invalid UUID: {}", id)})?;
    let coin_networks = state
        .coin_network_repo
        .list_by_coin_id(state.db_manager.get_connection().await?, id)
        .await?;

    Ok(Json(coin_networks.into_iter().map(CoinNetworkResponse::from).collect()))
}