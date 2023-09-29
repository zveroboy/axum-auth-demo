mod errors;

use self::errors::{Error, Result};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool<T: AsRef<str>>(db_connect_url: T, max: u32) -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(max)
        .acquire_timeout(Duration::from_millis(500))
        .connect(db_connect_url.as_ref())
        .await
        .map_err(|msg| Error::FailToCreatePool(msg.to_string()))
}
