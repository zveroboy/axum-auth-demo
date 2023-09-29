use axum_full_course::infrastructure::config::get_config;
use axum_full_course::infrastructure::store::{new_db_pool, Db};
use futures::stream::{self, StreamExt};
use std::fmt;
use std::fs;
use std::path::Path;

const SQL_DIR: &str = "sql/dev";
const RECREATE_SQL_DIR: &str = "sql/dev/00-reset.sql";

// struct DbConfig {
//     pub user: String,
//     pub host: String,
//     pub password: String,
//     pub database: String,
//     pub port: String,
// }

// impl DbConfig {
//     fn new() -> Self {
//         let pg_user = env::var("PGUSER").expect("PGUSER variable is required");
//         let pg_host = env::var("PGHOST").expect("PGHOST variable is required");
//         let pg_password = env::var("PGPASSWORD").expect("PGPASSWORD variable is required");
//         let pg_database = env::var("PGDATABASE").expect("PGDATABASE variable is required");
//         let pg_port = env::var("PGPORT").expect("PGPORT variable is required");

//         Self {
//             user: pg_user,
//             host: pg_host,
//             password: pg_password,
//             database: pg_database,
//             port: pg_port,
//         }
//     }

//     fn get_root_connection(&self) -> String {
//         let Self {
//             user,
//             host,
//             password,
//             database,
//             port,
//         } = self;

//         format!("postgres://{user}:{password}@{host}:{port}/{database}")
//     }

//     // fn get_admin_connection(&self) -> String {
//     //     let Self {
//     //         user: _user,
//     //         host,
//     //         password: _password,
//     //         database: _database,
//     //         port,
//     //     } = self;

//     //     let user = "app_user";
//     //     let password = "dev_only_pwd";
//     //     let database = "app_db";

//     //     format!("postgres://{user}:{password}@{host}:{port}/{database}")
//     // }
// }

// fn db_config() -> &'static DbConfig {
//     static PG_ROOT_CONNECTION: OnceLock<DbConfig> = OnceLock::new();

//     PG_ROOT_CONNECTION.get_or_init(DbConfig::new)
// }

async fn process_file<P>(db: &Db, path: P) -> Result<(), sqlx::Error>
where
    P: AsRef<Path> + fmt::Debug,
{
    println!("path: {path:?}");

    let content = fs::read_to_string(path).unwrap();

    let sql_iter = content.split(";").filter(|s| !s.is_empty());

    stream::iter(sql_iter)
        .then(|sql| sqlx::query(sql).execute(db))
        .collect::<Vec<_>>()
        .await;

    Ok(())
}

async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let conf = get_config();

    let db = new_db_pool(conf.db.get_connection(), 1).await?;
    process_file(&db, RECREATE_SQL_DIR).await?;

    // let db_with_admin_access = new_db_pool(conf.get_admin_connection()).await?;

    // these queries can be run for different DB users
    let mut paths = fs::read_dir(SQL_DIR)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.as_os_str() != RECREATE_SQL_DIR)
        .filter(|path| path.extension().filter(|&val| val == "sql").is_some())
        .collect::<Vec<_>>();

    paths.sort();

    stream::iter(paths)
        .then(|path| process_file(&db, path))
        .collect::<Vec<_>>()
        .await;

    // for path in paths {
    //     println!("path in loop {path:#?}");
    //     // process_file(&db, path).await?;
    // }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_db().await?;

    Ok(())
}
