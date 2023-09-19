use axum::response::Response;
use tracing::info;

pub async fn main_response_mapper(res: Response) -> Response {
    info!("main_response_mapper");
    res
}
