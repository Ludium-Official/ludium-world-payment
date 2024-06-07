pub mod log; 
pub mod near;
pub mod swagger;

use std::env;
use dotenvy::dotenv;
use tokio::sync::OnceCell;
use crate::adapter::output::persistence::db::_dev_utils;
use self::near::{KeyRotatingSignerWrapper, NearNetworkConfig};

#[derive(Debug, Clone)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug, Clone)]
struct DatabaseConfig {
    url: String,
}


#[derive(Debug, Clone)]
pub struct Config {
    run_mode: String,
    server: ServerConfig,
    db: DatabaseConfig,
    signer: KeyRotatingSignerWrapper,
    near_network_config: NearNetworkConfig,
}


impl Config {
    pub fn db_url(&self) -> &str {
        &self.db.url
    }

    pub fn server_host(&self) -> &str {
        &self.server.host
    }

    pub fn server_port(&self) -> u16 {
        self.server.port
    }

    pub fn signer(&self) -> KeyRotatingSignerWrapper {
        self.signer.clone()
    }

    pub fn near_network_config(&self) -> NearNetworkConfig {
        self.near_network_config.clone()
    }

    pub fn is_local(&self) -> bool {
        self.run_mode == "local"
    }
        
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();
    let run_mode = env::var("PAYMENT_RUN_MODE").unwrap_or_else(|_| "local".to_string());
    let env_file = format!(".env.{}", run_mode);
    dotenvy::from_filename(&env_file).ok();
    tracing::info!("RUN_MODE: {}", run_mode);

    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("PORT")
            .unwrap_or_else(|_| String::from("8090"))
            .parse::<u16>()
            .unwrap(),
    };

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let databse_name = if cfg!(test) {
        env::var("POSTGRES_TEST_DB").expect("POSTGRES_TEST_DB must be set")
    } else {
        env::var("POSTGRES_DB").expect("POSTGRES_DB must be set")
    };

    let database_config = DatabaseConfig {
        url: format!("{}/{}", db_url, databse_name)
    };

    let near_network_config = NearNetworkConfig::init();
    let signer = near_network_config.init_rotating_signer();

    if run_mode == "local" {
        // NOTE: Hardcode to prevent deployed system db update.
        let pg_host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let pg_port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
        let admin_database_url = format!("postgres://postgres:postgres@{}:{}/postgres", pg_host, pg_port);  
        _dev_utils::init_local(&database_config.url, &admin_database_url).await;
    }

    Config {
        run_mode,
        server: server_config,
        db: database_config,
        signer,
        near_network_config,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}