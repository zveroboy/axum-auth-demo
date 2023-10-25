use axum::http::Request;
use axum::middleware;
use axum::Router;
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::ServiceBuilderExt;
use tracing::info_span;

use super::config::Config;
use super::middleware::request_id::RequestIdHelper;
use super::middleware::user::auth_resolver;
use super::rest::static_router::static_router;
use super::state::AppState;
use super::store::Db;
use super::{auth, rest::hello::hello_router, ticket};

// Composition root
pub fn app_router(config: &Config, db: Db) -> Router {
    let app_state = AppState {
        config: config.clone(),
        db: db.clone(),
    };

    let request_id_service = ServiceBuilder::new()
        .set_x_request_id(RequestIdHelper::default())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // Log the matched route's path (with placeholders not filled in).
                    // Use request.uri() or OriginalUri if you want the real path.
                    let matched_path = request.uri().to_string();
                    // .extensions()
                    // .get::<MatchedPath>()
                    // .map(MatchedPath::as_str);
                    let headers = request
                        .headers()
                        .iter()
                        // .inspect(|(name, _)| println!("{}", name.as_str()))
                        .filter(|(name, _)| {
                            let header_name = name.as_str();
                            header_name == "x-request-id" || header_name == "cookie"
                        })
                        .fold(String::new(), |acc, (name, value)| {
                            format!("{acc} {name}: {value:#?}")
                        });

                    info_span!(
                        "request",
                        method = ?request.method(),
                        matched_path,
                        headers = headers
                        // some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(())
                .on_response(DefaultOnResponse::new().include_headers(true)),
        )
        // propagate `x-request-id` headers from request to response
        .propagate_x_request_id();

    // let auth_cookie_service = middleware::from_fn(auth_resolver);

    Router::new()
        .merge(hello_router())
        .nest("/auth", auth::router::auth_router())
        // TODO extract to different service
        .nest("/tickets", ticket::router::ticket_router())
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_resolver,
        )) // middleware call order: 1
        .layer(CookieManagerLayer::new()) // middleware call order: 0
        .layer(request_id_service)
        .fallback_service(static_router())
        .with_state(app_state)
}
