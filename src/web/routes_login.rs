use axum::{routing::post, Json, Router};
use serde::Deserialize;
use serde_json::Value;
use tower_cookies::{Cookie, Cookies};

use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:12} - api_login - {payload:?}", "HANDLER");

    // TODO: Implement real db/auth logic

    if payload.username != "lemon" || payload.pwd != "melon" {
        return Err(Error::LoginFail);
    }

    // TODO: Implement real token logic
    cookies.add(Cookie::new(crate::web::AUTH_TOKEN, "user-1.exp.sign"));

    let body = Json(serde_json::json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
