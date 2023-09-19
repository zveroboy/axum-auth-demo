use std::net::SocketAddr;
use std::sync::OnceLock; // use LazyLock::new(|| Client::new())

use axum_full_course::{ADDR_PORT, ADDR_URL};
use reqwest::Client;

static CLIENT: OnceLock<Client> = OnceLock::new();

fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| Client::new())
}

#[ignore]
#[tokio::test]
async fn router_hello() -> anyhow::Result<()> {
    let addr = SocketAddr::new(ADDR_URL, ADDR_PORT);

    let url = format!("http://{:#?}/hello", addr);

    let resp = get_client().get(url).send().await?.text().await?;

    assert_eq!(resp, "Hello, world!");

    Ok(())
}

#[ignore]
#[tokio::test]
async fn router_hello_query() -> anyhow::Result<()> {
    let addr = SocketAddr::new(ADDR_URL, ADDR_PORT);

    let url = format!("http://{:#?}/hello?name=Xenia", addr);

    let resp = get_client().get(url).send().await?.text().await?;

    assert_eq!(resp, "Hello, Xenia!");

    Ok(())
}

#[tokio::test]
async fn router_hello_path() -> anyhow::Result<()> {
    let addr = SocketAddr::new(ADDR_URL, ADDR_PORT);

    let url = format!("http://{:#?}/hello2/xenia", addr);

    let resp = get_client().get(url).send().await?.text().await?;

    assert_eq!(resp, "Hello, xenia!");

    Ok(())
}
