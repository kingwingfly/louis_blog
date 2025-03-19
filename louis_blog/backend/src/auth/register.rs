use super::error::{AuthErr, Result};
use crate::{config::Config, db::Db};
use axum::{
    body::Bytes,
    extract::State,
    response::{IntoResponse as _, Redirect, Response},
};
use hmac::{Hmac, Mac as _};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tracing::instrument;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterPayload {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(length(min = 8, max = 20))]
    pub password: String,
    #[validate(email)]
    pub email: String,
}

#[instrument(
    skip_all,
    fields(username, email),
    ret(level = "debug"),
    err(level = "warn")
)]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn register(State((config, db)): State<(Config, Db)>, body: Bytes) -> Result<Response> {
    let register_payload: RegisterPayload = if let Ok(payload) = serde_urlencoded::from_bytes(&body)
    {
        payload
    } else if let Ok(payload) = serde_json::from_slice(&body) {
        payload
    } else {
        return Err(AuthErr::Register {
            reason: "invalid payload".to_string(),
        });
    };
    tracing::Span::current()
        .record("username", &register_payload.username)
        .record("email", &register_payload.email);
    register_payload.validate().map_err(|e| AuthErr::Register {
        reason: e.to_string(),
    })?;
    let RegisterPayload {
        username,
        password,
        email,
    } = register_payload;
    let key = config.pwd_key.as_bytes();
    let salt = rand::random::<[u8; 32]>();
    let token_salt = rand::random::<[u8; 32]>();
    let mut hmac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    hmac.update(password.as_bytes());
    hmac.update(salt.as_ref());
    let hashed = &hmac.finalize().into_bytes()[..];
    db.register(username, email, hashed, &salt, &token_salt)
        .await
        .map_err(|_| AuthErr::Register {
            reason: "duplicated username or email".to_string(),
        })?;
    Ok(Redirect::to("/auth/login").into_response())
}
