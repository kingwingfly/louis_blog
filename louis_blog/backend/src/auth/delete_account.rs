use super::{
    error::{AuthErr, Result},
    AUTH_TOKEN,
};
use crate::{context::Context, db::Db};
use axum::{extract::State, Extension};
use tower_cookies::{Cookie, Cookies};
use tracing::instrument;

#[instrument(skip_all, fields(username=context.as_ref().map(|c| c.name.to_owned())))]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn delete_account(
    Extension(context): Extension<Option<Context>>,
    State(db): State<Db>,
    cookies: Cookies,
) -> Result<()> {
    match context {
        Some(context) => {
            db.delete_by_id(context.id)
                .await
                .map_err(|_| AuthErr::Token {
                    reason: "failed to delete account".to_string(),
                })?;
            cookies.remove(Cookie::from(AUTH_TOKEN));
            Ok(())
        }
        None => Err(AuthErr::Token {
            reason: "no token provided".to_string(),
        }),
    }
}
