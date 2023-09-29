use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;

use super::config::get_config;

// pub fn static_router() -> Router {
//     Router::new().nest_service("/", ServeDir::new("./"))
// }

async fn fallback_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not found")
}

pub fn static_router() -> MethodRouter {
    any_service(
        ServeDir::new(&get_config().web_folder).not_found_service(fallback_404.into_service()),
    )
}
