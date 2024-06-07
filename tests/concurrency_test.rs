use std::sync::{atomic::{AtomicUsize, Ordering}, Arc};
use anyhow::Result;
use serde_json::json;
use reqwest::{header::{HeaderMap, HeaderValue}, StatusCode};
use tokio::task::JoinHandle;
use uuid::Uuid;

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
        "random" => json!({
            "id": Uuid::new_v4().to_string(),
            "adm":true,"prv":true,"crt":true
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



struct TestResults {
    tx_approved: AtomicUsize,
    tx_failed: AtomicUsize,
    api_error: AtomicUsize,
}

impl TestResults {
    fn new() -> Self {
        Self {
            tx_approved: AtomicUsize::new(0),
            tx_failed: AtomicUsize::new(0),
            api_error: AtomicUsize::new(0),
        }
    }

    fn record(&self, status: &str) {
        match status {
            "TRANSACTION_APPROVED" => self.tx_approved.fetch_add(1, Ordering::Relaxed),
            "TRANSACTION_FAILED" => self.tx_failed.fetch_add(1, Ordering::Relaxed),
            "ApiError" => self.api_error.fetch_add(1, Ordering::Relaxed),
            _ => 0,
        };
    }

    fn print_summary(&self) {
        println!("Transaction Approved: {}", self.tx_approved.load(Ordering::Relaxed));
        println!("Transaction Failed: {}", self.tx_failed.load(Ordering::Relaxed));
        println!("Api Error: {}", self.api_error.load(Ordering::Relaxed));
    }
}

// region: --- 동시성 테스트 1

/**
 *  여러 미션에 여러 유저가 동시 클레임 요청
 *      
 * Total: 1000
        트랜잭션 재시도 10, 재시도 시간 1초
            Transaction Approved: 999
            Transaction Failed: 0
            Api Error: 1
        
        트랜잭션 재시도 7, 재시도 시간 1초
            Transaction Approved: 984
            Transaction Failed: 0
            Api Error: 16

        트랜잭션 재시도 5, 재시도 시간 1초
            Transaction Approved: 953
            Transaction Failed: 0
            Api Error: 47

 * Total: 500
        트랜잭션 재시도 10, 재시도 시간 1초
            Transaction Approved: 500
            Transaction Failed: 0
            Api Error: 0

        트랜잭션 재시도 7, 재시도 시간 1초
            Transaction Approved: 493
            Transaction Failed: 0
            Api Error: 7

        트랜잭션 재시도 5, 재시도 시간 1초
            Transaction Approved: 486
            Transaction Failed: 0
            Api Error: 14
 */

async fn create_random_user() -> Result<Arc<httpc_test::Client>> {
    let headers = create_headers("random");
    let client = reqwest::Client::builder()
        .default_headers(headers);
    let hc: Arc<httpc_test::Client> = Arc::new(httpc_test::new_client_with_reqwest(
        "http://localhost:8080",
        client
    )?);

    hc.do_post("/api/login", json!({
        "username": "demo1",
        "password": "welcome"
    })).await?;

    Ok(hc)
}

#[tokio::test]
#[ignore]
async fn test_mass_reward() -> Result<()> {
    let mut handles: Vec<JoinHandle<Result<String>>> = Vec::new();
    let mission_ids = [
        "a0008dda-0101-d2ff-a12d-b5bf10013812",
        "a0008dda-0101-d2ff-a12d-b5bf10013813",
        "a0008dda-0101-d2ff-a12d-b5bf10013814",
    ];
    
    let len = 500;

    let mut user_clients = Vec::new();
    for _ in 0..len {
        let client = create_random_user().await?;
        user_clients.push(client);
    }

    std::thread::sleep(std::time::Duration::from_secs(2));

    for i in 0..len {
        let hc_clone: Arc<httpc_test::Client> = Arc::clone(&user_clients[i]);
        let handle = tokio::task::spawn_blocking(move || {
            let mission_id = mission_ids[i % 3];
            tokio::runtime::Handle::current().block_on(send_reward_claim(hc_clone, &mission_id))
        });
        handles.push(handle);
    }

    let test_results = Arc::new(TestResults::new());
    for handle in handles {
        let result = handle.await??;
        test_results.record(&result);
    }

    test_results.print_summary();

    Ok(())
}

// endregion: --- 동시성 테스트 1

// region: --- 동시성 테스트 2: 악의적인 공격
/**
 *  Total: 1000, 동일 미션에 4명의 유저가 250번씩 악의적인 클레임 요청 
        Transaction Approved: 4
        Transaction Failed: 0
        Api Error: 996
        Good!
 */
#[tokio::test]
#[ignore]
async fn test_multi_reward() -> Result<()> {
    let mut handles: Vec<JoinHandle<Result<String>>> = Vec::new();

    let user_clients = [
        create_random_user().await?,
        create_random_user().await?,
        create_random_user().await?,
        create_random_user().await?,
    ];

    let len = 1000;
    for i in 0..len {
        let user_client = &user_clients[i % 4];
        let hc_clone: Arc<httpc_test::Client> = Arc::clone(&user_client);
        let handle = tokio::task::spawn_blocking(move || {
            let mission_id = "a0008dda-0101-d2ff-a12d-b5bf10013811";
            
            tokio::runtime::Handle::current().block_on(send_reward_claim(hc_clone, &mission_id))
        });

        handles.push(handle);
    }

    let test_results = Arc::new(TestResults::new());
    for handle in handles {
        let result = handle.await??;
        test_results.record(&result);
    }

    test_results.print_summary();
    Ok(())
}

// endregion: --- 동시성 테스트 2: 악의적인 공격

async fn send_reward_claim(hc: Arc<httpc_test::Client>, mission_id: &str) -> Result<String> {
    let response = hc.do_post("/api/reward-claims", json!({
        "mission_id": mission_id,
        // "coin_network_id": "22222222-0000-0000-0000-000000000001",
        "coin_network_id": "33333333-9c58-47f8-9a0f-2d0c8d3f807f",
        "amount": "0.00001",
        "user_address": "nomnomnom.testnet"
    })).await?;

    // let _ = response.print().await?;

    let body = response.json_body()?;
    
    if response.status() != StatusCode::CREATED {
        return Ok("ApiError".to_string());
    }

    let tx_status = body["reward_claim_status"].as_str().unwrap();

    Ok(tx_status.to_string())
}