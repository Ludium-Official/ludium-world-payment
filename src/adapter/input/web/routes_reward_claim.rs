use std::sync::Arc;
use axum::routing::get;
use axum::{extract::State, routing::post, Json, Router};
use crate::adapter::input::{ctx::Ctx, error::Error};
use crate::domain::model::reward_claim::{NewRewardClaimPayload, CombinedRewardClaimResponse};
use crate::AppState;
use crate::adapter::input::error::Result;
use uuid::Uuid;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/reward-claims", post(create_reward_claim))
        .route("/me/reward-claims", get(list_me_reward_claim))
        .with_state(state)
}

async fn create_reward_claim(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(new_reward_claim_payload): Json<NewRewardClaimPayload>,
) -> Result<Json<CombinedRewardClaimResponse>> {
    let user_id = Uuid::parse_str(ctx.user_info().user_id())
        .map_err(|_| Error::UUIDParsingError{ message: format!("invalid User UUID : {}", ctx.user_info().user_id())})?;
    let reward_claim = state.reward_claim_usecase.create_reward_claim(user_id, new_reward_claim_payload).await?;
    Ok(Json(CombinedRewardClaimResponse::from(reward_claim)))
}

async fn list_me_reward_claim(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> Result<Json<Vec<CombinedRewardClaimResponse>>> {
    let user_id = Uuid::parse_str(ctx.user_info().user_id())
        .map_err(|_| Error::UUIDParsingError{ message: format!("invalid User UUID : {}", ctx.user_info().user_id())})?;
    let reward_claims = state.reward_claim_usecase.get_me_reward_claim(user_id).await?;
    Ok(Json(reward_claims.into_iter().map(CombinedRewardClaimResponse::from).collect()))
}