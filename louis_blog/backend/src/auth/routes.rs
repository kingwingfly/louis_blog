use super::{login::login, logout::logout, register::register, whoami::whoami};
use crate::{config::Config, db::Db};
use axum::routing::{get, post, Router};

pub fn routes<S>(config: Config, db: Db) -> Router<S> {
    Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .route("/api/logout", get(logout))
        .route("/api/whoami", get(whoami))
        .with_state((config, db))
}
