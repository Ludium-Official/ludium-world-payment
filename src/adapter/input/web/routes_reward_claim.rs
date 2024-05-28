use std::sync::Arc;
use axum::{Router, Json, Extension, extract::{Path, State}, routing::{get, post, put, delete}};
use crate::{adapter::input::ctx::Ctx, domain::model::reward_claim::{RewardClaimApprovePayload, RewardClaimApproveResponse, RewardClaimResponse}};
use crate::domain::model::reward_claim::{NewRewardClaimPayload, CombinedRewardClaimResponse, RewardClaimStatus};
use crate::port::output::{DbManager, reward_claim_repository::RewardClaimRepository};
use crate::AppState;
use crate::adapter::input::error::{Error, Result};
use uuid::Uuid;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/reward_claims", post(create_reward_claim))
        .route("/reward_claims/batch", post(create_batch_reward_claims))
        .route("/reward_claims/:id/approve", put(approve_reward_claim))
        .route("/reward_claims/:id/reject", put(reject_reward_claim))
        .with_state(state)
}

async fn create_reward_claim(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<Ctx>,
    Json(new_reward_claim_payload): Json<NewRewardClaimPayload>,
) -> Result<Json<CombinedRewardClaimResponse>> {
    tracing::debug!("[handler] create_reward_claim {:?}", ctx);

    let reward_claim = state.reward_claim_usecase.create_reward_claim(new_reward_claim_payload).await?;
    Ok(Json(CombinedRewardClaimResponse::from(reward_claim)))
}

async fn create_batch_reward_claims(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<Ctx>,
    Json(new_reward_claim_payloads): Json<Vec<NewRewardClaimPayload>>,
) -> Result<Json<Vec<CombinedRewardClaimResponse>>> {
    tracing::debug!("[handler] create_batch_reward_claims {:?}", ctx);

    let reward_claims = state.reward_claim_usecase.create_multiple_reward_claims(new_reward_claim_payloads).await?;
    Ok(Json(reward_claims))
}

async fn reject_reward_claim(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<Ctx>,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<RewardClaimResponse>> {
    // TODO: is_admin?
    let reward_claim = state.reward_claim_usecase.reject_reward_claim(id).await?;
    Ok(Json(RewardClaimResponse::from(reward_claim)))
}

async fn approve_reward_claim(
    State(state): State<Arc<AppState>>,
    Extension(ctx): Extension<Ctx>,
    Path(id): Path<Uuid>,
    Json(payload): Json<RewardClaimApprovePayload>,
) -> Result<Json<RewardClaimApproveResponse>> {
    // TODO: is_admin?
    let reward_claim = state.reward_claim_usecase
        .approve_reward_claim(id, payload).await?;

    Ok(Json(reward_claim))
}