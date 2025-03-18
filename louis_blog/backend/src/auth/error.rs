use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum AuthErr {
    #[snafu(display("Failed to register: {reason}"))]
    Register { reason: String },
    #[snafu(display("Failed to login: {reason}"))]
    Login { reason: String },
    #[snafu(display("Token error: {reason}"))]
    Token { reason: String },
    #[snafu(display("Internal: {reason}"))]
    Internal { reason: String },
}

pub type Result<T> = core::result::Result<T, AuthErr>;

impl IntoResponse for AuthErr {
    fn into_response(self) -> Response {
        match self {
            AuthErr::Register { reason } => (StatusCode::BAD_REQUEST, reason),
            AuthErr::Login { reason } => (StatusCode::BAD_REQUEST, reason),
            AuthErr::Token { reason } => (StatusCode::BAD_REQUEST, reason),
            AuthErr::Internal { reason } => (StatusCode::INTERNAL_SERVER_ERROR, reason),
        }
        .into_response()
    }
}
