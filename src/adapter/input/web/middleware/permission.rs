use axum::body::Body;
use axum::middleware::Next;
use axum::http::Request;
use axum::response::Response;
use crate::adapter::input::ctx::Ctx;
use crate::adapter::input::error::{Error, Result};

pub async fn mw_require_auth(
	ctx: Result<Ctx>,
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
    match ctx {
        Ok(ctx) => {
            if ctx.is_authenticated() {
                Ok(next.run(req).await)
            } else {
                Err(Error::Unauthorized { message: "Authentication required".to_string()})
            }
        },
        Err(_) => Err(Error::Unauthorized { message: "Unauthorized".to_string()}),
    }
}
