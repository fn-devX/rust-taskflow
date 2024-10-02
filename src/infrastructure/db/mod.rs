use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use std::env;
use tokio_postgres::NoTls;
use thiserror::Error;
use deadpool::managed::PoolError as DeadpoolError;
use tokio_postgres::Error as PgError;

pub type DbPool = Pool;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Failed to create DB pool")]
    PoolError(#[from] DeadpoolError<PgError>),
    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),
    #[error("Failed to create DB pool config")]
    ConfigError(#[from] deadpool_postgres::config::ConfigError),
}

pub async fn init_db() -> Result<DbPool, DbError> {
    let mut cfg = Config::new();

    let db_url = env::var("DATABASE_URL")?;

    cfg.host = Some("localhost".to_string());
    cfg.dbname = Some("task_db".to_string());
    cfg.user = Some("postgres".to_string());
    cfg.password = Some("password".to_string());
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    Ok(pool)
}