pub mod log; 

use std::env;
use dotenvy::dotenv;
use tokio::sync::OnceCell;

use crate::adapter::output::persistence::db::_dev_utils;

#[derive(Debug)]
struct ServerConfig {
    host: String,
    port: u16,
}

#[derive(Debug)]
struct DatabaseConfig {
    url: String,
}

#[derive(Debug)]
pub struct Config {
    server: ServerConfig,
    db: DatabaseConfig,
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
}

pub static CONFIG: OnceCell<Config> = OnceCell::const_new();

async fn init_config() -> Config {
    dotenv().ok();

    let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_string());
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

    let databse_url_key = if cfg!(test) { "TEST_DATABASE_URL" } else { "DATABASE_URL" };
    let database_config = DatabaseConfig {
        url: env::var(databse_url_key).expect("DATABASE_URL must be set"),
    };

    if run_mode == "development" {
        // NOTE: Hardcode to prevent deployed system db update.
        let admin_database_url = "postgres://postgres:postgres@localhost:5432/postgres";
        _dev_utils::init_dev(&database_config.url, &admin_database_url).await;
    }

    Config {
        server: server_config,
        db: database_config,
    }
}

pub async fn config() -> &'static Config {
    CONFIG.get_or_init(init_config).await
}