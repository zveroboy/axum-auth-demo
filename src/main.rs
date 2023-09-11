use std::net::SocketAddr;

use axum::extract::{Path, Query};
use axum::middleware;
use axum::response::{Html, IntoResponse, Response};
use axum::{routing::get, Router};
use axum_full_course::{infrastructure, ADDR_PORT, ADDR_URL};
use serde::Deserialize;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::services::ServeDir;

// #[derive(Deserialize, Debug, Clone, Copy)]
// struct DemoParams<'a> {
//     // name: Option<&'a str>,
//     #[serde(borrow)]
//     name: &'a str,
// }

// impl<'a> Deserialize<'a> for DemoParams<'a>
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'a> {
//         todo!()
//     }
// }

// #[axum::debug_handler]
// async fn hello_demo_handler(Query(_params): Query<DemoParams<'static>>) -> &'static str
// // async fn hello_demo_handler() -> &'static str
// // where 
// //     T: 'static, 
// //     for<'a> Q: Query<DemoParams<'a, T>>
// {
//     // let params = DemoParams {
//     //     name: "aaa"
//     // };
//     // let res = params.name;
//     "res"
//     // let res = params.name.unwrap_or("world");
//     // Html(format!("{}", &res))
//     // Html(format!("Hello, {name}!"))
// }

// fn hello_demo_handler(Query(params): Query<DemoParams<'static>>) -> impl Future<Output = Response> + 'static
// // async fn hello_demo_handler() -> &'static str
// // where
// //     T: 'static,
// //     for<'a> Q: Query<DemoParams<'a, T>>
// {
//     // let params = DemoParams {
//     //     name: "aaa"
//     // };
//     async move {
//         let res = params.name.clone();
//         res.into_response()
//     }
//     // let res = params.name.unwrap_or("world");
//     // Html(format!("{}", &res))
//     // Html(format!("Hello, {name}!"))
// }

#[derive(Deserialize, Debug)]
struct HelloParams {
    name: Option<String>,
}

// #[axum::debug_handler]
async fn hello_handler(cookies: Cookies, Query(params): Query<HelloParams>) -> impl IntoResponse {

    println!("hello cookies {:?}", &cookies.list());

    let name = params.name.unwrap_or("world".to_string());
    Html(format!("Hello, {name}!"))
}

async fn hello_named_handler(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, {name}!"))
}

fn hello_router() -> Router {
    let router = Router::new();

    router
        // .route("/demo", get(hello_demo_handler))
        // .route("/hello2/:name", get(hello_named_handler))
        .route("/hello", get(hello_handler))
        .route("/hello2/:name", get(hello_named_handler))
}

fn static_router() -> Router {
    Router::new().nest_service("/", ServeDir::new("./"))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = SocketAddr::from((ADDR_URL, ADDR_PORT));
    let router_all = Router::new()
        .merge(hello_router())
        .nest("/auth", infrastructure::login::router::auth_router())
        .layer(middleware::map_response(main_response_mapper)) // middleware call order: 1
        .layer(CookieManagerLayer::new()) // middleware call order: 0
        .fallback_service(static_router());

    axum::Server::bind(&addr)
        .serve(router_all.into_make_service())
        .await?;

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response{
    println!("-> main_response_mapper");
    println!();
    res
}