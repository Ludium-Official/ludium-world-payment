#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

#[ignore]
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    hc.do_get("/hello").await?.print().await?;

    // login 
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "password": "welcome"
    })).await?.print().await?;

    // user 
    hc.do_post("/api/users", json!({
            "nick": "quick_user_1",
            "self_intro": "hello! i'm quick_user_1",
            "phn_nmb": "010-1112-6672"
        })).await?.print().await?;
    hc.do_get("/api/users").await?.print().await?;
    hc.do_get("/api/users/quick_user_1").await?.print().await?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn quick_dev2() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8090")?;

    hc.do_get("/hello").await?.print().await?;

    // login
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "password": "welcome"
    })).await?.print().await?;

    // coin
    hc.do_get("/api/coins").await?.print().await?;
    hc.do_get("/api/coins/a3d281dd-4f85-4e5e-b639-b5bf1d8ee853").await?.print().await?;

    // network 
    hc.do_get("/api/networks").await?.print().await?;

    // coin_network
    hc.do_get("/api/coins/a3d281dd-4f85-4e5e-b639-b5bf1d8ee853/networks").await?.print().await?;
    hc.do_get("/api/coins/5cb2dca4-b693-49b5-bd20-00ddce72d54b/networks").await?.print().await?;


    Ok(())
}