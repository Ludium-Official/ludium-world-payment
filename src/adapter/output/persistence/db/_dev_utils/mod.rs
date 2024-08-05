mod dev_db;

use tokio::sync::OnceCell;
use crate::config::config;
use super::postgres::PostgresDbManager;

/// Initialize environment for local development.
pub async fn init_local(database_url: &str, admin_database_url: &str) {
	static INIT: OnceCell<()> = OnceCell::const_new();
	INIT.get_or_init(|| async {
		dev_db::init_local_db(database_url, admin_database_url).await.unwrap();
	})
	.await;
}

// test 
#[allow(unused)]
pub async fn init_test() -> PostgresDbManager {
	static DB_MANAGER: OnceCell<PostgresDbManager> = OnceCell::const_new();
    let config = config().await;

	// hard check
	if !config.db_url().contains("temp_test"){
		panic!("db_url should contain 'temp_test'");
	}

    DB_MANAGER.get_or_init(|| async {
        PostgresDbManager::new(&config.db_url(), config.db_connection_size()).await.unwrap()
    }).await;

    DB_MANAGER.get().unwrap().clone()
}