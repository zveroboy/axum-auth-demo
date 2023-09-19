use std::net::SocketAddr;
use std::sync::{Arc, OnceLock}; // use LazyLock::new(|| Client::new())

use axum_full_course::infrastructure::auth::{dto::LoginDto, AUTH_TOKEN};
use axum_full_course::{ADDR_PORT, ADDR_URL};

use axum::http::{HeaderValue, StatusCode};
use reqwest::cookie::CookieStore;
use reqwest::header::SET_COOKIE;
use reqwest::{cookie, Client, ClientBuilder, IntoUrl, Response};

static ADDR: SocketAddr = SocketAddr::new(ADDR_URL, ADDR_PORT);

// static CLIENT: OnceLock<Client> = OnceLock::new();

// fn get_client() -> &'static Client {
//     CLIENT.get_or_init(|| Client::new())
// }

fn print_resp(resp: &Response) {
    println!("{:#?}", resp.headers());
    println!("{:#?}", resp.cookies().collect::<Vec<_>>());
}

#[tokio::test]
async fn should_login() -> anyhow::Result<()> {
    let url: reqwest::Url = format!("http://{:#?}/auth/login", ADDR).parse()?;

    let cookie_provider = Arc::new(cookie::Jar::default());

    let client = ClientBuilder::new()
        .cookie_store(true)
        .cookie_provider(cookie_provider.clone())
        .build()?;

    let resp = client
        .post(url.clone())
        .json(&LoginDto {
            email: "demo".to_string(),
            password: "test".to_string(),
        })
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::OK);
    print_resp(&resp);

    let url: reqwest::Url = format!("http://{:#?}/hello", ADDR).parse()?;

    // let cookie = &cookie_provider.cookies(&url);
    // println!("{} {:?}", &url, cookie);

    let resp = client.get(url).send().await?;
    print_resp(&resp);

    Ok(())
}

#[ignore]
#[tokio::test]
async fn should_not_login() -> anyhow::Result<()> {
    let url = format!("http://{:#?}/auth/login", ADDR);

    let resp = Client::new()
        .post(url)
        .json(&LoginDto {
            email: "demo".to_string(),
            password: "wrong".to_string(),
        })
        .send()
        .await?;

    assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

    Ok(())
}
