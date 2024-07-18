use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{extract::State, routing::post, Json, Router};
use crate::adapter::input::{ctx::Ctx, error::Error};
use crate::domain::model::reward_claim::{CombinedRewardClaimResponse, NewRewardClaimPayload};
use crate::AppState;
use crate::adapter::input::error::Result;
use uuid::Uuid;

pub fn routes(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/reward-claims", post(create_reward_claim))
        .route("/me/reward-claims", get(list_me_reward_claim))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/api/reward-claims",
    request_body = NewRewardClaimPayload,
    responses(
        (status = 201, description = "Create reward claim", body = Vec<CombinedRewardClaimResponse>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "Not Found", body = ErrorResponse),
        (status = 409, description = "Conflict", body = ErrorResponse),
        (status = 500, description = "Internel Server Error", body = ErrorResponse)
    ),
    tag = "RewardClaim"
)]
pub async fn create_reward_claim(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
    Json(new_reward_claim_payload): Json<NewRewardClaimPayload>,
) -> Result<impl IntoResponse> {
    let user_id = Uuid::parse_str(ctx.user_info().user_id())
        .map_err(|_| Error::UUIDParsingError{ message: format!("invalid User UUID : {}", ctx.user_info().user_id())})?;

    let reward_claim = state.reward_claim_usecase.create_reward_claim(user_id, new_reward_claim_payload).await?;
    Ok((StatusCode::CREATED, Json(CombinedRewardClaimResponse::from(reward_claim))))
}

#[utoipa::path(
    get,
    path = "/api/me/reward-claims",
    responses(
        (status = 200, description = "List of my reward claims", body = Vec<CombinedRewardClaimResponse>),
        (status = 400, description = "Bad Request", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 500, description = "Internel Server Error", body = ErrorResponse)
    ),
    tag = "RewardClaim"
)]
pub async fn list_me_reward_claim(
    State(state): State<Arc<AppState>>,
    ctx: Ctx,
) -> Result<Json<Vec<CombinedRewardClaimResponse>>> {
    let user_id = Uuid::parse_str(ctx.user_info().user_id())
        .map_err(|_| Error::UUIDParsingError{ message: format!("invalid User UUID : {}", ctx.user_info().user_id())})?;
    let reward_claims = state.reward_claim_usecase.get_me_reward_claim(user_id).await?;
    Ok(Json(reward_claims.into_iter().map(CombinedRewardClaimResponse::from).collect()))
}