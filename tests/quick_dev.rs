#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;
use tokio::fs;

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

    // reward_claims
    hc.do_post("/api/reward_claims", json!({
        "mission_id": "a00081dd-0000-deff-abcd-b5bf10000859",
        "coin_network_id": "1859ebb9-d031-473a-8241-b0b6832c2652",
        "amount": 100.0,
        "user_id": "bcd28999-2abc-0abc-1abc-b5bf1d8ee999",
        "user_address": "won999.near"
    })).await?.print().await?;
    // hc.do_get("/api/reward_claims").await?.print().await?;
    // hc.do_get("/api/reward_claims/1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d").await?.print().await?;

    hc.do_post("/api/reward_claims/batch", json!(
        [
            {   
                "mission_id": "a00081dd-0000-deff-abcd-b5bf10000859",
                "coin_network_id": "1859ebb9-d031-473a-8241-b0b6832c2652",
                "amount": 100.0,
                "user_id": "bcd28999-2abc-0abc-1abc-b5bf1d8ee888",
                "user_address": "won999.near"
            },
            {   
                "mission_id": "a00081dd-0000-deff-abcd-b5bf10004459",
                "coin_network_id": "3e6d84d8-9c58-47f8-9a0f-2d0c8d3f807f",
                "amount": 100.35,
                "user_id": "bcd28999-2abc-0abc-1abc-b5bf1d8ee999",
                "user_address": "test.near"
            },
        ]
    )).await?.print().await?;

    hc.do_put("/api/reward_claims/1a2b3c4d-5e6f-7a8b-9c0d-1e2f3a4b5c6d/reject", json!({})).await?.print().await?;

    // --- relay test 
    let payload: serde_json::Value = serde_json::from_str(
        &fs::read_to_string("tests/data/approve_payload.json").await?
    )?;
    hc.do_put("/api/reward_claims/3c4d5e6a-7a8b-9c0d-1e2a-3a41111d0000/approve", payload).await?.print().await?;

    Ok(())
}
