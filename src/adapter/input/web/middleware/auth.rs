use std::sync::Arc;

use crate::adapter::input::ctx::{Ctx, UserInfo};
use crate::web::{ACCESS_TOKEN, GOOGLE_ID};
use crate::AppState;
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::http::HeaderMap;
use axum::middleware::Next;
use axum::response::Response;
use tower_cookies::{Cookie, Cookies};

use crate::adapter::input::error::{Error, Result};

pub async fn mw_ctx_resolver(
	_state: State<Arc<AppState>>,
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response> {
	let access_token = cookies.get(ACCESS_TOKEN).map(|c| c.value().to_string());
    let ggl_id = cookies.get(GOOGLE_ID).map(|c| c.value().to_string());

    let headers: &HeaderMap = req.headers();
	let user_info = headers.get("x-user-right")
        .and_then(|value| {
            value.to_str().ok().map(|s| {
                serde_json::from_str::<UserInfo>(s).map_err(|e| {
                    tracing::error!("Failed to parse x-user-right: {}", e);
                    e
                }).ok()
            }).flatten()
        });

	let result_ctx = match (user_info, access_token, ggl_id) {
        (Some(user_info), Some(access_token), Some(ggl_id)) => {
			tracing::debug!("[middleware] mw_ctx_resolver - Ok");
			Ok(Ctx::new(user_info, access_token, ggl_id))
        }
        _ => Err(Error::AuthFailNoAuthInformation),
    };

	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(Error::AuthFailNoAuthInformation))
	{
		cookies.remove(Cookie::from(ACCESS_TOKEN));
        cookies.remove(Cookie::from(GOOGLE_ID));
	}

	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}

// endregion: --- Ctx Extractor
