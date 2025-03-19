use super::{error::Result, AUTH_TOKEN};
use tower_cookies::{Cookie, Cookies};

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn logout(cookies: Cookies) -> Result<()> {
    cookies.remove(Cookie::from(AUTH_TOKEN));
    Ok(())
}
