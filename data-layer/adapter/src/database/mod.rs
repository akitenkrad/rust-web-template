pub mod models;

use shared::config::DatabaseConfig;
use sqlx::{PgPool, postgres::PgConnectOptions};

fn make_pu_connect_options(cfg: &DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

#[derive(Debug, Clone)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with(cfg: &DatabaseConfig) -> ConnectionPool {
    ConnectionPool(PgPool::connect_lazy_with(make_pu_connect_options(cfg)))
}
