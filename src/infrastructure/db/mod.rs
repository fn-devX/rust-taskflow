use tokio_postgres::{NoTls, Error};
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use std::env;

pub type DbPool = Pool;

pub async fn init_db() -> Result<DbPool, Error> {
    let mut cfg = Config::new();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    cfg.host = Some("localhost".to_string());
    cfg.dbname = Some("task_db".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("password".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(NoTls)?;

    Ok(pool)
}