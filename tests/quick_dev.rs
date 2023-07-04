use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;

    hc.do_get("/hello2/melon").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        serde_json::json!({
            "username": "lemon",
            "pwd": "melon"
        }
        ),
    );

    req_login.await?.print().await?;

    Ok(())
}
