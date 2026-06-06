use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::config::DatabaseConfig;

pub fn create_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(&config.database_url)
}
