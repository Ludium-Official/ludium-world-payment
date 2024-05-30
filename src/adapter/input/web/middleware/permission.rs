use axum::body::Body;
use axum::middleware::Next;
use axum::http::Request;
use axum::response::Response;
use crate::adapter::input::ctx::Ctx;
use crate::adapter::input::error::{Error, Result};

pub async fn mw_require_admin(
    ctx: Result<Ctx>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    tracing::debug!("[middleware] require_admin = {ctx:?}");
    match ctx {
        Ok(ctx) => {
            if ctx.is_admin() {
                Ok(next.run(req).await)
            } else {
                Err(Error::AdminUnauthorized { message: "Admin rights required".to_string()})
            }
        },
        Err(_) => Err(Error::AdminUnauthorized { message: "Unauthorized".to_string()}),
    }
}
