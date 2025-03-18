use super::error::Result;
use crate::context::Context;
use axum::{Extension, Json};
use tracing::instrument;

#[instrument(skip_all, ret(level = "debug"), err(level = "warn"))]
#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn whoami(Extension(ctx): Extension<Context>) -> Result<Json<Context>> {
    Ok(Json(ctx))
}
