use super::error::{AuthErr, Result};
use crate::context::Context;
use axum::{Extension, Json};
use tracing::instrument;

#[instrument(skip_all, ret(level = "debug"), err(level = "warn"))]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn whoami(Extension(context): Extension<Option<Context>>) -> Result<Json<Context>> {
    match context {
        Some(ctx) => Ok(Json(ctx)),
        None => Err(AuthErr::Token {
            reason: "token unreachable".to_string(),
        }),
    }
}
