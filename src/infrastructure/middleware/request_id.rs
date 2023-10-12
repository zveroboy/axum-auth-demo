use axum::response::{IntoResponse, Response};
use uuid::Uuid;

pub async fn set_request_id(response: Response) -> impl IntoResponse {
    let request_id = Uuid::new_v4().to_string();

    ([("x-request-id", request_id)], response)
}
