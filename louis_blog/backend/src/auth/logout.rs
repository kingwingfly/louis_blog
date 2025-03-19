use super::{error::Result, AUTH_TOKEN};
use tower_cookies::{Cookie, Cookies};

pub async fn logout(cookies: Cookies) -> Result<()> {
    cookies.remove(Cookie::from(AUTH_TOKEN));
    Ok(())
}
