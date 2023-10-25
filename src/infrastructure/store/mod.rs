mod errors;

use self::errors::{Error, Result};

use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;

pub type Db = PgPool;

pub async fn new_db_pool(db_connect_url: &str, max: u32) -> Result<Db> {
    const CONNECTION_TIMEOUT: u64 = 500;

    PgPoolOptions::new()
        .max_connections(max)
        .acquire_timeout(Duration::from_millis(CONNECTION_TIMEOUT))
        .connect(db_connect_url)
        .await
        .map_err(|msg| Error::FailToCreatePool(msg.to_string()))
}
