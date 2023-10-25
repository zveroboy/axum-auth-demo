use std::net::SocketAddr;

use axum_full_course::infrastructure::app_router::app_router;
use axum_full_course::infrastructure::config;
use axum_full_course::infrastructure::store::new_db_pool;
use axum_full_course::{ADDR_PORT, ADDR_URL};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// TODO: Take from config
static ADDR: SocketAddr = SocketAddr::new(ADDR_URL, ADDR_PORT);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "axum_full_course=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().with_target(false))
        .init();

    let config = config::get_config();
    let db = new_db_pool(config.db.get_connection().as_str(), 1).await?;

    let router_all = app_router(config, db);

    info!("LISTENING {addr}", addr = ADDR);

    axum::Server::bind(&ADDR)
        .serve(router_all.into_make_service())
        .await
        .expect("Startup crash");

    Ok(())
}
