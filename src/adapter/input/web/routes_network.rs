use std::sync::Arc;
use axum::extract::State;
use axum::{Router, Json, extract::Path, routing::get};
use crate::adapter::input::ctx::Ctx;
use crate::domain::model::network::{NetworkResponse, NewNetworkPayload};
use crate::port::output::network_repository::NetworkRepository;
use crate::port::output::DbManager;
use crate::AppState;
use crate::adapter::input::error::{Error, Result};
use uuid::Uuid; 

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/networks",  get(list_networks))
        .route("/networks/:id", get(get_network))
        .with_state(state)
}

#[allow(unused)]
async fn create_network(
	State(state): State<Arc<AppState>>,
    _ctx: Ctx,
	Json(new_network_payload): Json<NewNetworkPayload>,
) -> Result<Json<NetworkResponse>> {
	let network = state.network_repo.insert(
        state.db_manager.get_connection().await?,
        new_network_payload,
    ).await?;

	Ok(Json(NetworkResponse::from(network)))
}

async fn list_networks(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
) -> Result<Json<Vec<NetworkResponse>>> {
    let networks = state
        .network_repo
        .list(state.db_manager.get_connection().await?)
        .await?;
    Ok(Json(networks.into_iter().map(NetworkResponse::from).collect()))
}

async fn get_network(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Path(id): Path<String>
) -> Result<Json<NetworkResponse>> {
    let id = Uuid::parse_str(&id).map_err(|_| Error::UUIDParsingError{ message: format!("Invalid UUID: {}", id)})?;
    let network = state
        .network_repo
        .get(state.db_manager.get_connection().await?, id)
        .await?;

    Ok(Json(NetworkResponse::from(network)))
}
