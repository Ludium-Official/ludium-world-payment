use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;
use std::time::Duration;

use deadpool_diesel::postgres::{Manager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

// NOTE: Hardcode to prevent deployed system db update.
// sql files
const SQL_RECREATE_DB_FILE_NAME: &str = "00-recreate-db.sql";
const SQL_DIR: &str = "scripts/dev_initial";
const DEMO_PWD: &str = "welcome";
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("scripts/dev_initial/");

pub async fn init_dev_db(database_url: &str, admin_database_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("{:} - init_dev_db()", "[DEV-ONLY]");
    let sql_dir = get_sql_dir()?;
    
    recreate_db(admin_database_url, &sql_dir).await?;
    execute_sql_files(database_url, &sql_dir).await?;

    Ok(())
}

fn get_sql_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let components: Vec<_> = current_dir.components().collect();
    let base_dir = if components.iter().any(|c| c.as_os_str() == "crates") {
        components[..components.len() - 3].iter().collect::<PathBuf>()
    } else {
        current_dir.clone()
    };
    Ok(base_dir.join(SQL_DIR))
}

async fn recreate_db(admin_database_url: &str, sql_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let sql_recreate_db_file = sql_dir.join(SQL_RECREATE_DB_FILE_NAME);
    let root_db = new_db_pool(admin_database_url)?;
    pexec(&root_db, &sql_recreate_db_file).await
}

async fn execute_sql_files(database_url: &str, sql_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let app_db = new_db_pool(database_url)?;
    let mut paths: Vec<PathBuf> = fs::read_dir(sql_dir)?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();
    paths.sort();

    for path in paths {
        let path_str = path.to_string_lossy();

        if path_str.ends_with(".sql")
            && !path_str.ends_with(SQL_RECREATE_DB_FILE_NAME)
        {
            pexec(&app_db, &path).await?;
        }
    }

    Ok(())
}

async fn pexec(db: &Pool, file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    info!("{:} - pexec: {:?}", "[DEV-ONLY]", file);

    let content = fs::read_to_string(file)?;
    let sqls: Vec<String> = content.split(';').map(|s| s.to_string()).collect();
    
	let conn = db.get().await?;
    conn.interact(move |conn| {
        for sql in &sqls {
            if !sql.trim().is_empty() {
                diesel::sql_query(sql).execute(conn).map_err(|e| {
                    println!("pexec error while running:\n{}", sql);
                    println!("cause:\n{}", e);
                    e
                })?;
            }
        }
        Ok::<_, diesel::result::Error>(())
    }).await??;

    Ok(())
}

pub fn new_db_pool(db_con_url: &str) -> Result<Pool, Box<dyn std::error::Error>> {
	let manager = Manager::new(
        db_con_url.to_string(),
        deadpool_diesel::Runtime::Tokio1,
    );
    let pool = Pool::builder(manager).build().unwrap();

    Ok(pool)
}
