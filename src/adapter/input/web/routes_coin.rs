use std::sync::Arc;
use axum::extract::State;
use axum::{Router, Json, Extension, extract::Path, routing::get, routing::post};
use crate::adapter::input::ctx::Ctx;
use crate::domain::model::coin::{Coin, CoinResponse, NewCoinPayload};
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
	Extension(ctx): Extension<Ctx>,
	Json(new_coin_payload): Json<NewCoinPayload>,
) -> Result<Json<CoinResponse>> {
	tracing::debug!("[handler] create_coin {:?}", ctx);

	let coin = state.coin_repo.insert(
        state.db_manager.get_connection().await?,
        new_coin_payload,
    ).await?;

	Ok(Json(CoinResponse::from(coin)))
}

async fn list_coins(
    State(state): State<Arc<AppState>>,
	Extension(ctx): Extension<Ctx>,
) -> Result<Json<Vec<CoinResponse>>> {
    tracing::debug!("[handler] list_coins");

    let coins = state
        .coin_repo
        .list(state.db_manager.get_connection().await?)
        .await?;
    Ok(Json(coins.into_iter().map(CoinResponse::from).collect()))
}

async fn get_coin(
    State(state): State<Arc<AppState>>,
	Extension(ctx): Extension<Ctx>,
    Path(id): Path<String>
) -> Result<Json<CoinResponse>> {
    tracing::debug!("[handler] get_coin");

    let id = Uuid::parse_str(&id).map_err(|_| Error::InputInvalid{ field: "id".to_string(), message: "Invalid UUID".to_string()})?;
    let coin = state
        .coin_repo
        .get(state.db_manager.get_connection().await?, id)
        .await?;

    Ok(Json(CoinResponse::from(coin)))
}


async fn get_coin_networks_by_coin_id(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<Ctx>,
    Path(id): Path<String>
) -> Result<Json<Vec<CoinNetworkResponse>>> {
    tracing::debug!("[handler] get_coin_networks_by_coin_id");

    let id = Uuid::parse_str(&id).map_err(|_| Error::InputInvalid{ field: "id".to_string(), message: "Invalid UUID".to_string()})?;
    let coin_networks = state
        .coin_network_repo
        .list_by_coin_id(state.db_manager.get_connection().await?, id)
        .await?;

    Ok(Json(coin_networks.into_iter().map(CoinNetworkResponse::from).collect()))
}