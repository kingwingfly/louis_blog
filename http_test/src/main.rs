mod auth;

use anyhow::Result;
use auth::auth;

#[tokio::main]
async fn main() -> Result<()> {
    auth().await?;
    Ok(())
}
