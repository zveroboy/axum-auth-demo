use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::middleware;
use axum::response::{Html, IntoResponse};
use axum::{routing::get, Router};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tracing::info;

use crate::domain::model::TicketService;

use super::auth::middleware::auth_resolver;
use super::static_router::static_router;
use super::store::new_db_pool;
use super::ticket::router::TicketAppState;
use super::ticket::service::PgTicketRepository;
use super::{auth, config, ticket};

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

#[derive(Deserialize, Debug)]
struct HelloParams {
    name: Option<String>,
}

// #[axum::debug_handler]
// #[tracing::instrument(name="handle_hello")]
async fn handle_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    // info!(headers = format!("{headers:#?}"));

    // let cs = cookies.list().iter().fold(String::new(), |acc, c| format!("{}\n{:#?}", acc, c));

    // let span = span!(tracing::Level::INFO, "my_span", cookies = cs);

    // let _ = span.enter();

    let name = params.name.unwrap_or("world".to_string());

    info!(name = name);
    Html(format!("Hello, {name}!"))
}

async fn handle_hello_named(Path(name): Path<String>) -> impl IntoResponse {
    Html(format!("Hello, {name}!"))
}

pub fn hello_router() -> Router {
    let router = Router::new();

    router
        // .route("/demo", get(hello_demo_handler))
        // .route("/hello2/:name", get(hello_named_handler))
        .route("/hello", get(handle_hello))
        .route("/hello2/:name", get(handle_hello_named))
}

// endregion: Hello world

// Composition root
pub async fn app_router() -> Router {
    let config = config::get_config();
    let db = new_db_pool(config.db.get_connection(), 1)
        .await
        .expect("Unable to connect to create db pool");

    let ticket_state = TicketAppState {
        ticket_service: TicketService::new(PgTicketRepository::new(db)),
    };

    Router::new()
        .merge(hello_router())
        .nest("/auth", auth::router::auth_router())
        // .layer(middleware::from_fn(add_auth_ctx))
        .nest(
            "/tickets",
            ticket::router::ticket_router()
                // .route_layer(middleware::from_fn(auth::middleware::require_auth))
                .with_state(ticket_state),
        )
        .layer(middleware::map_response(
            super::middleware::main_response_mapper,
        ))
        .layer(middleware::from_fn(auth_resolver)) // middleware call order: 1
        .layer(CookieManagerLayer::new()) // middleware call order: 0
        .layer(
            tower_http::trace::TraceLayer::new_for_http()
                // .make_span_with(|request: &Request<_>| {
                //     // Log the matched route's path (with placeholders not filled in).
                //     // Use request.uri() or OriginalUri if you want the real path.
                //     let matched_path = request.uri().to_string();
                //     // .extensions()
                //     // .get::<MatchedPath>()
                //     // .map(MatchedPath::as_str);
                //     let headers = request
                //         .headers()
                //         .iter()
                //         .fold(String::new(), |acc, (name, value)| {
                //             format!("{acc} {name}: {value:#?}")
                //         });
                //     info_span!(
                //         "request",
                //         method = ?request.method(),
                //         matched_path,
                //         headers = headers
                //         // some_other_field = tracing::field::Empty,
                //     )
                // })
                .on_request(())
                .on_response(()),
        )
        .fallback_service(static_router())
}
