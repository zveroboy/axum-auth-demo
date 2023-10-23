use std::net::SocketAddr;

use axum_full_course::infrastructure::auth::dto::{LoginDto, RegisterDto};
use axum_full_course::{ADDR_PORT, ADDR_URL};

use axum::http::StatusCode;
use reqwest::{Client, Response};

static ADDR: SocketAddr = SocketAddr::new(ADDR_URL, ADDR_PORT);

fn print_resp(resp: &Response) {
    println!("{:#?}", resp);
    // println!("{:#?}", resp.headers());
    // println!("{:#?}", resp.cookies().collect::<Vec<_>>());
}

#[tokio::test]
async fn should_register_and_login() -> anyhow::Result<()> {
    let register_url: reqwest::Url = format!("http://{:#?}/auth/register", ADDR).parse()?;

    let auth_url: reqwest::Url = format!("http://{:#?}/auth/login", ADDR).parse()?;

    // let cookie_provider = Arc::new(cookie::Jar::default());

    // let client = ClientBuilder::new()
    //     .cookie_store(true)
    //     // .cookie_provider(cookie_provider.clone())
    //     .build()?;

    let client = Client::new();

    let register_dto = RegisterDto {
        email: "demo_13".to_string(),
        password: "test".to_string(),
    };
    let resp = client.post(register_url).json(&register_dto).send().await?;
    print_resp(&resp);
    assert_eq!(resp.status(), StatusCode::OK);

    let login_dto = LoginDto {
        email: register_dto.email.clone(),
        password: register_dto.password.clone(),
    };
    let resp = client.post(auth_url).json(&login_dto).send().await?;

    assert_eq!(resp.status(), StatusCode::OK);

    let auth_token_res = resp.cookies().find(|cookie| cookie.name() == "auth-token");
    assert!(auth_token_res.is_some());

    // print_resp(&resp);

    // let url: reqwest::Url = format!("http://{:#?}/hello", ADDR).parse()?;

    // let cookie = &cookie_provider.cookies(&url);
    // println!("{} {:?}", &url, cookie);

    // let resp = client.get(url).send().await?;
    // print_resp(&resp);

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
