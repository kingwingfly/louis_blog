use super::{error::Result, AUTH_TOKEN};
use axum::response::{IntoResponse as _, Redirect, Response};
use tower_cookies::{Cookie, Cookies};

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn logout(cookies: Cookies) -> Result<Response> {
    cookies.remove(Cookie::from(AUTH_TOKEN));
    Ok(Redirect::to("/").into_response())
}
