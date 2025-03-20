use crate::{
    auth::{validate_token, AUTH_TOKEN},
    config::Config,
    db::Db,
};
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub id: i32,
    pub name: String,
}

pub async fn mw_context(
    State((config, db)): State<(Config, Db)>,
    cookies: Cookies,
    mut req: Request,
    next: Next,
) -> Response {
    match validate_token(&cookies, &config.token_key, &db).await {
        Ok(context) => {
            req.extensions_mut().insert(Some(context));
        }
        Err(_) => {
            req.extensions_mut().insert(None::<Context>);
            cookies.remove(Cookie::from(AUTH_TOKEN));
        }
    }
    next.run(req).await
}
