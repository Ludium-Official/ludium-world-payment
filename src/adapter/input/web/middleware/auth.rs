use crate::adapter::input::ctx::Ctx;
use crate::web::AUTH_TOKEN;
use crate::AppState;
use axum::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use lazy_regex::regex_captures;
use tower_cookies::{Cookie, Cookies};

use crate::adapter::input::error::{Error, Result};

pub async fn mw_require_auth(
	ctx: Result<Ctx>,
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
	tracing::debug!("[middlware] mw_require_auth = {ctx:?}");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolver(
	_state: State<AppState>,
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response> {
	tracing::debug!("[middlware] mw_ctx_resolver");

	let auth_token = cookies.get(AUTH_TOKEN).map(|c| c.value().to_string());

	tracing::debug!("[auth_token] {auth_token:?}");

	let result_ctx = match auth_token
		.ok_or(Error::AuthFailNoAuthTokenCookie)
		.and_then(parse_token)
	{
		Ok((user_id, _exp, _sign)) => {
			// TODO: Token components validations.
			Ok(Ctx::new(user_id))
		}
		Err(e) => Err(e),
	};

	if result_ctx.is_err()
		&& !matches!(result_ctx, Err(Error::AuthFailNoAuthTokenCookie))
	{
		cookies.remove(Cookie::from(AUTH_TOKEN))
	}

	req.extensions_mut().insert(result_ctx);

	Ok(next.run(req).await)
}

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Ctx {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		tracing::debug!("[extractor] Ctx");

		parts
			.extensions
			.get::<Result<Ctx>>()
			.ok_or(Error::AuthFailCtxNotInRequestExt)?
			.clone()
	}
}

// endregion: --- Ctx Extractor

/// Parse a token of format `user-[user-id].[expiration].[signature]`
/// Returns (user_id, expiration, signature)
fn parse_token(token: String) -> Result<(u64, String, String)> {
	let (_whole, user_id, exp, sign) = regex_captures!(
		r#"^user-(\d+)\.(.+)\.(.+)"#, // a literal regex
		&token
	)
	.ok_or(Error::AuthFailTokenWrongFormat)?;

	let user_id: u64 = user_id
		.parse()
		.map_err(|_| Error::AuthFailTokenWrongFormat)?;

	tracing::debug!("[parse_token] - {user_id} - {exp} - {sign}");
	Ok((user_id, exp.to_string(), sign.to_string()))
}