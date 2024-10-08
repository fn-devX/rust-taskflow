use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use std::env;
use tokio_postgres::NoTls;
use thiserror::Error;
use deadpool::managed::PoolError as DeadpoolError;
use tokio_postgres::Error as PgError;
use std::str::FromStr;

pub type DbPool = Pool;

#[derive(Debug, Error)]
pub enum DbError {
    #[error("Failed to create DB pool")]
    PoolError(#[from] DeadpoolError<PgError>),
    #[error("Environment variable error: {0}")]
    EnvError(#[from] env::VarError),
    #[error("Failed to create DB pool config")]
    ConfigError(#[from] deadpool_postgres::ConfigError),
    #[error("Invalid database URL format")]
    InvalidUrl,
}

pub async fn init_db() -> Result<DbPool, DbError> {
    let mut cfg = Config::new();

    let db_url = env::var("DATABASE_URL")
        .map_err(|_| DbError::EnvError(env::VarError::NotPresent))?;

    let db_uri = url::Url::from_str(&db_url).map_err(|_| DbError::InvalidUrl)?;

    cfg.host = db_uri.host_str().map(|h| h.to_string());
    cfg.port = db_uri.port();
    cfg.dbname = db_uri.path_segments().map(|s| s.collect::<Vec<_>>().join("/")).map(|p| p.trim_start_matches('/').to_string());
    cfg.user = db_uri.username().to_string().into();
    cfg.password = db_uri.password().map(|p| p.to_string());

    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).expect("Failed to create pool");

    Ok(pool)
}
