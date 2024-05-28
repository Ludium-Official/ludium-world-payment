//! DELETE_ME: Test only 

use std::sync::Arc;

use crate::adapter::input::ctx::Ctx;

use crate::adapter::input::error::{Result, Error};
use crate::domain::model::user::{NewUserPayload, UserResponse};
use crate::port::output::{DbManager, UserRepository};
use crate::AppState;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Extension, Json, Router};

pub fn routes(state: Arc<AppState>) -> Router {
	Router::new()
		.route("/users", get(list_users).post(create_user))
		.route("/users/:nickname", get(get_user_by_nickname))
		.with_state(Arc::clone(&state))
}

// region:    --- REST Handlers
async fn create_user(
	State(state): State<Arc<AppState>>,
	Extension(ctx): Extension<Ctx>,
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
	Extension(ctx): Extension<Ctx>,
) -> Result<Json<Vec<UserResponse>>> {
	tracing::debug!("[handler] list_users");

	let users = state.user_repo.list(
        state.db_manager.get_connection().await?
    ).await?;

	println!("users: {:?}", users);

	Ok(Json(users.into_iter().map(UserResponse::from).collect()))
}

async fn get_user_by_nickname(
	State(state): State<Arc<AppState>>,
	Extension(ctx): Extension<Ctx>,
	Path(nickname): Path<String>,
) -> Result<Json<UserResponse>> {
	tracing::debug!("[handler] get_user_by_nickname");

    let user = state.user_repo.first_by_nick(
        state.db_manager.get_connection().await?,
        nickname.clone()
    ).await?;

	if let Some(user) = user {
		Ok(Json(UserResponse::from(user)))
	} else {
		Err(Error::UserNicknameNotFound { nickname })
	}
}

// endregion: --- REST Handlers