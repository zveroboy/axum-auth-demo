// region: Hello world

// #[derive(Deserialize, Debug)]
// struct DemoParams {
//     // name: Option<&'a str>,
//     // #[serde(borrow)]
//     name: String,
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
// async fn hello_demo_handler<'a>(Query(mut params): Query<DemoParams>) -> &'a str
// // async fn hello_demo_handler() -> &'static str
// // where
// //     T: 'static,
// //     for<'a> Q: Query<DemoParams<'a, T>>
// {
//     // let params = DemoParams {
//     //     name: "aaa"
//     // };
//     // let res = &params.name;
//     let res = std::mem::take(&mut params.name);
//     res.leak()
//     // params.name.as_str()
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

use axum::{
    extract::{Path, Query, State},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;
use tracing::info;

use crate::{
    domain::user::error::Error,
    infrastructure::{middleware::error::AppError, state::AppState},
};

#[derive(Deserialize, Debug)]
pub struct HelloParams {
    name: Option<String>,
}
async fn hello_demo_handler(// Path(user_id): Path<Uuid>,
    // State(user_repo): State<DynUserRepo>,
) -> Result<String, AppError> {
    // let user = user_repo.find(user_id).await?;

    // Ok(user.into())
    Err(Error::FailToLogin)?
}

// #[axum::debug_handler]
// #[tracing::instrument(name="handle_hello")]
async fn handle_hello(
    State(state): State<AppState>,
    Query(params): Query<HelloParams>,
) -> impl IntoResponse {
    let name = params.name.unwrap_or("world".to_string());

    info!(name = name);
    Html(format!("Hello, {name} from {}!", state.foo))
}

async fn handle_hello_named(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, {name}!"))
}

pub fn hello_router() -> Router<AppState> {
    let router = Router::new();

    router
        .route("/demo", get(hello_demo_handler))
        // .route("/hello2/:name", get(hello_named_handler))
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handle_hello_named))
}

// endregion: Hello world
