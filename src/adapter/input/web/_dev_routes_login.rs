//! DELETE_ME: Test only 

use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};
use utoipa::ToSchema;

use crate::web;
use crate::adapter::input::error::{Error, Result};
use std::env;
use dotenvy::dotenv;

pub fn routes() -> Router {
    Router::new().route("/api/login", post(login))
}

#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginPayload,
    tag = "Login"
)]
async fn login(
    cookies: Cookies, 
    payload: Json<LoginPayload>
) -> Result<Json<Value>> {
    tracing::debug!("[handler] api_login - {payload:?}");

    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::Unknown);
    }   
    dotenv().ok();
    let env_file = ".env.cookie";
    dotenvy::from_filename(env_file).ok();
    let access_token = env::var("TEST_ACCESS_TOKEN").unwrap_or_else(|_| "set access_token".to_string());
    let google_id = env::var("TEST_GOOGLE_ID").unwrap_or_else(|_| "set google_id".to_string());

    let mut access_token_cookie = Cookie::new(web::ACCESS_TOKEN, access_token);
    access_token_cookie.set_http_only(true);
    access_token_cookie.set_path("/");  
    cookies.add(access_token_cookie);

    let mut google_id_cookie = Cookie::new(web::GOOGLE_ID, google_id);
    google_id_cookie.set_http_only(true); 
    google_id_cookie.set_path("/");  
    cookies.add(google_id_cookie);

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginPayload {
    username: String,
    password: String,
}