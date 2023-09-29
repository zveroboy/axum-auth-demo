use super::util::get_env_required;

pub struct DbConfig {
    pub user: String,
    pub host: String,
    pub password: String,
    pub database: String,
    pub port: String,
}

impl DbConfig {
    pub fn new() -> Self {
        let pg_user = get_env_required("PGUSER");
        let pg_host = get_env_required("PGHOST");
        let pg_password = get_env_required("PGPASSWORD");
        let pg_database = get_env_required("PGDATABASE");
        let pg_port = get_env_required("PGPORT");

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
