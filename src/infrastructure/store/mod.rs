mod errors;

use self::errors::{Error, Result};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool<T: AsRef<str>>(db_connect_url: T, max: u32) -> Result<Db> {
    const CONNECTION_TIMEOUT: u64 = 500;

    PgPoolOptions::new()
        .max_connections(max)
        .acquire_timeout(Duration::from_millis(CONNECTION_TIMEOUT))
        .connect(db_connect_url.as_ref())
        .await
        .map_err(|msg| Error::FailToCreatePool(msg.to_string()))
}

// TODO: Delete everything down below

// #[async_trait]
// trait BaseTx {
//     async fn get_keys(&mut self) -> Result<u8>;
// }

// trait Datab {
//     type Tx: BaseTx;
// }

// trait AbsDriver: Send + Sync {
//     type Transaction: Tx + Send + Sync;
// }

// #[derive(Clone)]
// struct Driver<D>
// where
//     D: Datab + Clone + Send + Sync + 'static,
//     D::Tx: BaseTx + Send + Sync + 'static,
// {
//     db: D,
// }

// impl<D> Driver<D>
// where
//     D: Datab + Clone + Send + Sync + 'static,
//     D::Tx: BaseTx + Send + Sync + 'static,
// {
//     async fn get_keys(self) -> Result<u8> {
//         // let mut tx = self.db.begin().await?;
//         // let keys = tx.get_keys().await?;
//         // tx.commit().await?;
//         Ok(12)
//     }
// }
