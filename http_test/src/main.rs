use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    let hc = httpc_test::new_client("http://127.0.0.1:3000")?;
    let resp = hc
        .do_post(
            "/api/register",
            json!({
            "username":"Louis",
            "password":"12345678",
            "email":"louis@gamil.com",
            }),
        )
        .await?;
    resp.print().await?;
    assert_eq!(resp.status(), 200);

    let resp = hc
        .do_post(
            "/api/login",
            json!({
            "email":"louis@gamil.com",
            "password":"12345678",
            }),
        )
        .await?;
    resp.print().await?;
    assert_eq!(resp.status(), 200);

    let resp = hc.do_get("/api/whoami").await?;
    resp.print().await?;
    assert_eq!(resp.json_body()?["name"], "Louis");

    let resp = hc
        .do_post(
            "/api/login",
            json!({
            "email":"louis@gamil.com",
            "password":"11111111",
            }),
        )
        .await?;
    resp.print().await?;
    assert_eq!(resp.status(), 400);
    assert!(resp.text_body()?.contains("wrong password"));

    let resp = hc
        .do_post(
            "/api/login",
            json!({
            "email":"david@gamil.com",
            "password":"12345678",
            }),
        )
        .await?;
    resp.print().await?;
    assert_eq!(resp.status(), 400);
    assert!(resp.text_body()?.contains("user not found"));

    let resp = hc.do_get("/api/not_exist").await?;
    assert_eq!(resp.status(), 404);

    Ok(())
}
