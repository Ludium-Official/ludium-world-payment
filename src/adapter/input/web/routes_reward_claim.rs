use std::sync::Arc;
use axum::{Router, Json, Extension, extract::{Path, State}, routing::{get, post, put, delete}};
use crate::adapter::input::ctx::Ctx;
use crate::domain::model::reward_claim::{NewRewardClaimPayload, RewardClaimResponse, RewardClaimStatus};
use crate::port::output::{DbManager, reward_claim_repository::RewardClaimRepository};
use crate::AppState;
use crate::adapter::input::error::{Error, Result};
use uuid::Uuid;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/reward_claims", post(create_reward_claim))
        .route("/reward_claims/batch", post(create_batch_reward_claims))
        // .route("/reward_claims", get(list_reward_claims))
        // .route("/reward_claims/:id", get(get_reward_claim))
        .with_state(state)
}

async fn create_reward_claim(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<Ctx>,
    Json(new_reward_claim_payload): Json<NewRewardClaimPayload>,
) -> Result<Json<RewardClaimResponse>> {
    tracing::debug!("[handler] create_reward_claim {:?}", ctx);

    let reward_claim = state.reward_claim_usecase.create_reward_claim(new_reward_claim_payload).await?;
    Ok(Json(RewardClaimResponse::from(reward_claim)))
}

async fn create_batch_reward_claims(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<Ctx>,
    Json(new_reward_claim_payloads): Json<Vec<NewRewardClaimPayload>>,
) -> Result<Json<Vec<RewardClaimResponse>>> {
    tracing::debug!("[handler] create_batch_reward_claims {:?}", ctx);

    let reward_claims = state.reward_claim_usecase.create_multiple_reward_claims(new_reward_claim_payloads).await?;
    Ok(Json(reward_claims))
}

// async fn get_reward_claim(
//     State(state): State<Arc<AppState>>,
//     Extension(ctx): Extension<Ctx>,
//     Path(id): Path<String>,
// ) -> Result<Json<RewardClaimResponse>> {
//     tracing::debug!("[handler] get_reward_claim");

//     let id = Uuid::parse_str(&id).map_err(|_| Error::InputInvalid { field: "id".to_string(), message: "Invalid UUID".to_string() })?;
//     let reward_claim = state
//         .reward_claim_repo
//         .get(state.db_manager.get_connection().await?, id)
//         .await?;

//     Ok(Json(RewardClaimResponse::from(reward_claim)))
// }

// async fn list_reward_claims(
//     State(state): State<Arc<AppState>>,
//     Extension(ctx): Extension<Ctx>,
// ) -> Result<Json<Vec<RewardClaimResponse>>> {
//     tracing::debug!("[handler] list_reward_claims");

//     let reward_claims = state
//         .reward_claim_repo
//         .list(state.db_manager.get_connection().await?)
//         .await?;
//     Ok(Json(reward_claims.into_iter().map(RewardClaimResponse::from).collect()))
// }