use std::sync::Arc;
use axum::{extract::{Path, State}, middleware, routing::{post, put}, Json, Router};
use crate::{adapter::input::{ctx::Ctx, error::Error}, domain::model::reward_claim::{RewardClaimApprovePayload, RewardClaimApproveResponse, RewardClaimResponse}};
use crate::domain::model::reward_claim::{NewRewardClaimPayload, CombinedRewardClaimResponse};
use crate::AppState;
use crate::adapter::input::error::Result;
use uuid::Uuid;

use super::middleware::permission::mw_require_admin;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/reward-claims", post(create_reward_claim))
        .route("/reward-claims/batch", post(create_batch_reward_claims))
        .route("/reward-claims/:id/approve", put(approve_reward_claim).layer(middleware::from_fn(mw_require_admin)))
        .route("/reward-claims/:id/reject", put(reject_reward_claim).layer(middleware::from_fn(mw_require_admin)))
        .with_state(state)
}

async fn create_reward_claim(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Json(new_reward_claim_payload): Json<NewRewardClaimPayload>,
) -> Result<Json<CombinedRewardClaimResponse>> {
    let reward_claim = state.reward_claim_usecase.create_reward_claim(new_reward_claim_payload).await?;
    Ok(Json(CombinedRewardClaimResponse::from(reward_claim)))
}

async fn create_batch_reward_claims(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Json(new_reward_claim_payloads): Json<Vec<NewRewardClaimPayload>>,
) -> Result<Json<Vec<CombinedRewardClaimResponse>>> {
    let reward_claims = state.reward_claim_usecase.create_multiple_reward_claims(new_reward_claim_payloads).await?;
    Ok(Json(reward_claims))
}

async fn reject_reward_claim(
    State(state): State<Arc<AppState>>,
    _ctx: Ctx,
    Path(id): Path<Uuid>,
    Json(_payload): Json<serde_json::Value>,
) -> Result<Json<RewardClaimResponse>> {
    let reward_claim = state.reward_claim_usecase.reject_reward_claim(id).await?;
    Ok(Json(RewardClaimResponse::from(reward_claim)))
}

async fn approve_reward_claim(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Path(id): Path<Uuid>,
    Json(payload): Json<RewardClaimApprovePayload>,
) -> Result<Json<RewardClaimApproveResponse>> {
    let user_id = Uuid::parse_str(ctx.user_info().user_id())
        .map_err(|_| Error::UUIDParsingError{ message: format!("Invalid UUID: {}", id)})?;
    let reward_claim = state.reward_claim_usecase
        .approve_reward_claim(user_id, id, payload).await?;

    Ok(Json(reward_claim))
}