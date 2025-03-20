use super::{
    delete_account::delete_account, login::login, logout::logout, register::register,
    whoami::whoami,
};
use crate::{config::Config, db::Db};
use axum::routing::{get, post, Router};

pub fn routes<S: Send + Sync + Clone + 'static>(config: Config, db: Db) -> Router<S> {
    Router::new()
        .route("/api/register", post(register))
        .route("/api/login", post(login))
        .with_state((config, db.clone()))
        .route("/api/delete_account", post(delete_account))
        .with_state(db)
        .route("/api/logout", get(logout))
        .route("/api/whoami", get(whoami))
}
