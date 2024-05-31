//! DELETE_ME: Test only 

use std::sync::Arc;

use crate::adapter::input::ctx::Ctx;

use crate::adapter::input::error::Result;
use crate::domain::model::user::{NewUserPayload, UserResponse};
use crate::port::output::{DbManager, UserRepository};
use crate::AppState;
use axum::extract::State;
use axum::routing::get;
use axum::{Json, Router};

use super::middleware::permission::mw_require_auth;

pub fn routes(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/users", 
			get(list_users).layer(axum::middleware::from_fn(mw_require_auth))
			.post(create_user))
		.with_state(Arc::clone(&state))
}

// region:    --- REST Handlers
async fn create_user(
	State(state): State<Arc<AppState>>,
	ctx: Ctx,
	Json(new_user_payload): Json<NewUserPayload>,
) -> Result<Json<UserResponse>> {
	tracing::debug!("[handler] create_user {:?}", ctx);

	let user = state.user_repo.insert(
        state.db_manager.get_connection().await?,
        new_user_payload,
    ).await?;

	Ok(Json(UserResponse::from(user)))
}

async fn list_users(
	State(state): State<Arc<AppState>>,
	ctx: Ctx,
) -> Result<Json<Vec<UserResponse>>> {
	tracing::debug!("[handler] list_users {:?}", ctx);

	let users = state.user_repo.list(
        state.db_manager.get_connection().await?
    ).await?;

	println!("users: {:?}", users);

	Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

// endregion: --- REST Handlers