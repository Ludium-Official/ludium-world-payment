#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    hc.do_get("/hello").await?.print().await?;

    // login 
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "password": "welcome"
    })).await?.print().await?;

    // ticket
    // hc.do_post("/api/tickets", json!({
    //     "title": "Ticket 1"
    // })).await?.print().await?;

    // hc.do_delete("/api/tickets/0").await?.print().await?;
    // hc.do_get("/api/tickets").await?.print().await?;

    hc.do_post("/api/users", json!({
            "nick": "quick_user_1",
            "self_intro": "hello! i'm quick_user_1",
            "phn_nmb": "010-1112-6672"
        })).await?.print().await?;
    hc.do_get("/api/users").await?.print().await?;

    hc.do_get("/api/users/quick_user_1").await?.print().await?;


    Ok(())
}

#[tokio::test]
async fn quick_dev2() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    hc.do_get("/hello").await?.print().await?;

    hc.do_get("/payment").await?.print().await?;

    Ok(())
}