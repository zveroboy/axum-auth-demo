use std::net::SocketAddr;
use std::sync::{OnceLock, Arc}; // use LazyLock::new(|| Client::new())

use axum_full_course::infrastructure::login::{dto::LoginDto, AUTH_TOKEN};
use axum_full_course::{ADDR_PORT, ADDR_URL};

use axum::http::{StatusCode, HeaderValue};
use reqwest::cookie::{CookieStore};
use reqwest::header::SET_COOKIE;
use reqwest::{Client, ClientBuilder, cookie, IntoUrl};

// static CLIENT: OnceLock<Client> = OnceLock::new();

// fn get_client() -> &'static Client {
//     CLIENT.get_or_init(|| Client::new())
// }

#[tokio::test]
async fn should_login() -> anyhow::Result<()> {
    let addr = SocketAddr::from((ADDR_URL, ADDR_PORT));

    let url: reqwest::Url = format!("http://{:#?}/auth/login", addr).parse()?;

    let cookie_provider = Arc::new(cookie::Jar::default());
    // let cookie_provider = 

    let client = ClientBuilder::new()
        // .host
        // .cookie_store(true)
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

    // let auth_cookie = resp.cookies().find(|c| c.name() == AUTH_TOKEN);
    // println!("domain {}", &url.host_str().unwrap());

    // let cookies_domains = &cookie_provider.;
    let cookie = &cookie_provider.cookies(&url);
    println!("{} {:?}", &url, cookie);


// /    let dom = auth_cookie.as_ref().unwrap().domain();

    // dom

    // println!("{:?}", auth_cookie.as_ref().map(|c| c.value()));


    // let resp = client
    //     .post(url)
    //     .json(&LoginDto {
    //         email: "demo".to_string(),
    //         password: "test".to_string(),
    //     })
    //     .send()
    //     .await?;


    let url: reqwest::Url = format!("http://{:#?}/hello", addr).parse()?;

    
    // let url = format!("http://{:#?}/hello", addr);
    // println!("{:?}", resp.cookies().collect::<Vec<_>>());

    fn extract_response_cookie_headers<'a>(
        headers: &'a axum::http::HeaderMap,
    ) -> impl Iterator<Item = &'a HeaderValue> + 'a {
        headers.get_all(SET_COOKIE).iter()
    }

    // It doesn't work otherwise
    let mut cookies =
        extract_response_cookie_headers(&resp.headers()).peekable();
    if cookies.peek().is_some() {
        cookie_provider.set_cookies(&mut cookies, &url);
    }


    let cookie = &cookie_provider.cookies(&url);
    println!("{} {:?}", &url, cookie);

    // let cookie = &cookie_provider.set_cookies(&url);
    // println!("{} {:?}", &url, cookie);
    
    let get = client.get(url);
    // println!("{}", get.);
    let resp = get.send().await?;
    
    // let auth_cookie = resp.cookies().find(|c| c.name() == AUTH_TOKEN);

    // println!("{:?}", auth_cookie.as_ref().map(|c| c.value()));
    // println!("{:?}", resp.cookies().collect::<Vec<_>>());

    Ok(())
}

#[ignore]
#[tokio::test]
async fn should_not_login() -> anyhow::Result<()> {
    let addr = SocketAddr::from((ADDR_URL, ADDR_PORT));

    let url = format!("http://{:#?}/auth/login", addr);

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
