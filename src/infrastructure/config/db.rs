#[derive(Clone)]
pub struct DbConfig {
    pub user: String,
    pub host: String,
    pub password: String,
    pub database: String,
    pub port: String,
}

impl DbConfig {
    pub fn new(
        pg_user: String,
        pg_host: String,
        pg_password: String,
        pg_database: String,
        pg_port: String,
    ) -> Self {
        Self {
            user: pg_user,
            host: pg_host,
            password: pg_password,
            database: pg_database,
            port: pg_port,
        }
    }

    pub fn get_connection(&self) -> String {
        let Self {
            user,
            host,
            password,
            database,
            port,
        } = self;

        format!("postgres://{user}:{password}@{host}:{port}/{database}")
    }
}

// fn db_config() -> &'static DbConfig {
//     static PG_ROOT_CONNECTION: OnceLock<DbConfig> = OnceLock::new();

//     PG_ROOT_CONNECTION.get_or_init(DbConfig::new)
// }
