use anyhow::Result;
use serde_json::json;
use reqwest::header::{HeaderMap, HeaderValue};

fn create_headers(header_type: &str,
) -> HeaderMap {
    let x_user_right_value = match header_type {
        "admin" => json!({
            "id": "00000000-0000-0000-0000-000000000001",
            "adm":true,"prv":true,"crt":true
        }),
        "provider" => json!({
            "id": "00000000-0000-0000-0000-000000000002",
            "adm":false,"prv":true,"crt":true
        }),
        "contributor" => json!({
            "id": "00000000-0000-0000-0000-000000000003",
            "adm":false,"prv":false,"crt":true
        }),
        _ => json!({
            "id": "00000000-0000-0000-0000-000000000003",
            "adm":false,"prv":false,"crt":true
        }),
    };

    let mut headers = HeaderMap::new();

    headers.insert("x-user-right", HeaderValue::from_str(&x_user_right_value.to_string()).unwrap());
    headers
}

#[ignore]
#[tokio::test]
async fn quick_dev() -> Result<()> {
    let headers = create_headers("admin");
    let client = reqwest::Client::builder()
        .default_headers(headers);
    
    let hc = httpc_test::new_client_with_reqwest(
        "http://localhost:8080",
        client
    )?;

    hc.do_get("/hello").await?.print().await?;

    // login 
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "password": "welcome"
    })).await?.print().await?;

    // get apis 
    hc.do_get("/api/coins").await?.print().await?;
    hc.do_get("/api/coin-networks?network_code=Near").await?.print().await?;
    hc.do_get("/api/me/reward-claims").await?.print().await?;

    Ok(())
}

#[ignore]
#[tokio::test]
async fn quick_reward() -> Result<()> {
    let headers = create_headers("provider");
    let client = reqwest::Client::builder()
        .default_headers(headers);
    
    let hc = httpc_test::new_client_with_reqwest(
        "http://localhost:8080",
        client
    )?;

    hc.do_get("/hello").await?.print().await?;

    // login
    hc.do_post("/api/login", json!({
        "username": "demo1",
        "password": "welcome"
    })).await?.print().await?;

    // reward_claims
    hc.do_post("/api/reward-claims", json!({
        "mission_id": "10000000-0000-0000-0000-000000000001",
        "coin_network_id": "22222222-0000-0000-0000-000000000001", // usdt
        "amount": "0.00001",
        "user_address": "nomnomnom.testnet"
    })).await?.print().await?;

    hc.do_post("/api/reward-claims", json!({
        "mission_id": "10000000-0000-0000-0000-000000000002",
        "coin_network_id": "22222222-0000-0000-0000-000000000001", // usdt
        "amount": "0.00001",
        "user_address": "nomnomnom.testnet"
    })).await?.print().await?;

    hc.do_post("/api/reward-claims", json!({
        "mission_id": "10000000-0000-0000-0000-000000000003",
        "coin_network_id": "33333333-9c58-47f8-9a0f-2d0c8d3f807f", // near 
        "amount": "0.00001",
        "user_address": "nomnomnom.testnet"
    })).await?.print().await?;

    hc.do_get("/api/me/reward-claims").await?.print().await?;
    Ok(())
}

