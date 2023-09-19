use std::net::SocketAddr;

use axum_full_course::domain::errors::Result;
use axum_full_course::infrastructure::app::app_router;
use axum_full_course::{ADDR_PORT, ADDR_URL};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

static ADDR: SocketAddr = SocketAddr::new(ADDR_URL, ADDR_PORT);

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "axum_full_course=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router_all = app_router();

    info!("LISTENING {addr}", addr = ADDR);

    axum::Server::bind(&ADDR)
        .serve(router_all.into_make_service())
        .await
        .expect("Startup crash");

    Ok(())
}
