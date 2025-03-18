use super::error::{AuthErr, Result};
use super::token::generate_token;
use super::AUTH_TOKEN;
use crate::db::entity::user;
use crate::{config::Config, db::Db};
use axum::extract::State;
use axum::Json;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tower_cookies::{Cookie, Cookies};
use tracing::instrument;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 20))]
    pub password: String,
}

#[instrument(skip_all, fields(email=login_payload.email), err(level = "warn"))]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn login(
    State((config, db)): State<(Config, Db)>,
    cookies: Cookies,
    Json(login_payload): Json<LoginPayload>,
) -> Result<()> {
    login_payload.validate().map_err(|e| AuthErr::Register {
        reason: e.to_string(),
    })?;
    let LoginPayload { email, password } = login_payload;
    let user::Model {
        id,
        secret,
        salt,
        token_salt,
        ..
    } = db
        .get_by_email(email)
        .await
        .map_err(|_| AuthErr::Internal {
            reason: "database unreachable".to_string(),
        })?
        .ok_or(AuthErr::Login {
            reason: "user not found".to_string(),
        })?;
    let key = config.pwd_key.as_bytes();
    let mut mac = Hmac::<Sha256>::new_from_slice(key).unwrap();
    mac.update(password.as_bytes());
    mac.update(salt.as_ref());
    let hashed = &mac.finalize().into_bytes()[..];
    if secret != hashed {
        return Err(AuthErr::Login {
            reason: "wrong password".to_string(),
        });
    }
    let token = generate_token(&config.token_key, token_salt, config.token_duration_sec, id);
    cookies.add(Cookie::new(AUTH_TOKEN, token));
    Ok(())
}
