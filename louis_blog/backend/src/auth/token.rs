use super::{
    error::{AuthErr, Result},
    AUTH_TOKEN,
};
use crate::{
    config::Config,
    context::Context,
    db::{entity::user, Db},
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use base64::{prelude::BASE64_STANDARD, Engine};
use core::fmt;
use hmac::{Hmac, Mac as _};
use sha2::Sha256;
use std::str::FromStr;
use tower_cookies::{Cookie, Cookies};

pub async fn mw_context(
    State((config, db)): State<(Config, Db)>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    match validate_token(&cookies, &config.token_key, &db).await {
        Ok(context) => {
            req.extensions_mut().insert(context);
        }
        Err(_) => cookies.remove(Cookie::from(AUTH_TOKEN)),
    }

    Ok(next.run(req).await)
}

pub struct Token {
    identifier: String,
    expiration_time: String,
    token: String,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}",
            self.identifier, self.expiration_time, self.token
        )
    }
}

impl FromStr for Token {
    type Err = AuthErr;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('.').collect();
        if parts.len() != 3 {
            return Err(AuthErr::Token {
                reason: "invalid token format".to_string(),
            });
        }
        Ok(Token {
            identifier: parts[0].to_string(),
            expiration_time: parts[1].to_string(),
            token: parts[2].to_string(),
        })
    }
}

pub fn generate_token(
    token_key: impl AsRef<[u8]>,
    token_salt: impl AsRef<[u8]>,
    token_duration_sec: u64,
    identifier: i32,
) -> String {
    let expiration_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + token_duration_sec;
    let mut hmac = Hmac::<Sha256>::new_from_slice(token_key.as_ref()).unwrap();
    hmac.update(format!("{}.{}", identifier, expiration_time).as_bytes());
    hmac.update(token_salt.as_ref());
    let token = BASE64_STANDARD.encode(hmac.finalize().into_bytes());
    let identifier = BASE64_STANDARD.encode(identifier.to_ne_bytes());
    let expiration_time = BASE64_STANDARD.encode(expiration_time.to_ne_bytes());
    Token {
        identifier,
        expiration_time,
        token,
    }
    .to_string()
}

pub async fn validate_token(
    cookies: &Cookies,
    token_key: impl AsRef<[u8]>,
    db: &Db,
) -> Result<Context> {
    let token = cookies
        .get(AUTH_TOKEN)
        .ok_or(AuthErr::Token {
            reason: "no auth token".to_string(),
        })?
        .value()
        .to_string();

    let Token {
        identifier,
        expiration_time,
        token,
    } = Token::from_str(&token)?;

    let identifier = i32::from_ne_bytes(
        BASE64_STANDARD
            .decode(identifier.as_bytes())
            .map_err(|_| AuthErr::Token {
                reason: "invalid token".to_string(),
            })?
            .try_into()
            .map_err(|_| AuthErr::Token {
                reason: "invalid token".to_string(),
            })?,
    );

    let expiration_time = u64::from_ne_bytes(
        BASE64_STANDARD
            .decode(expiration_time.as_bytes())
            .map_err(|_| AuthErr::Token {
                reason: "invalid token".to_string(),
            })?
            .try_into()
            .map_err(|_| AuthErr::Token {
                reason: "invalid token".to_string(),
            })?,
    );

    let user::Model {
        name, token_salt, ..
    } = db
        .get_by_id(identifier)
        .await
        .map_err(|_| AuthErr::Internal {
            reason: "database unreachable".to_string(),
        })?
        .ok_or(AuthErr::Token {
            reason: "id in token not found".to_string(),
        })?;

    let mut hmac = Hmac::<Sha256>::new_from_slice(token_key.as_ref()).unwrap();
    hmac.update(format!("{}.{}", identifier, expiration_time).as_bytes());
    hmac.update(token_salt.as_ref());
    let expect = BASE64_STANDARD.encode(hmac.finalize().into_bytes());

    if token != expect {
        return Err(AuthErr::Token {
            reason: "invalid token".to_string(),
        });
    }

    if expiration_time
        < std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    {
        return Err(AuthErr::Token {
            reason: "token expired".to_string(),
        });
    }
    Ok(Context {
        id: identifier,
        name,
    })
}
