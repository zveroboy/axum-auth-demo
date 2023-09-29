use axum::response::{IntoResponse, Response};
use tracing::error;
use uuid::Uuid;

use crate::domain::errors::Error;

pub async fn main_response_mapper(res: Response) -> Response {
    let err = res
        .extensions()
        .get::<Error>()
        .map(|e| e.client_status_code_and_error());

    if let Some((status, client_err)) = &err {
        error!("response_mapper: {} {:?}", status, client_err);
    }

    let err_response = err.map(|(status_code, client_error)| {
        (status_code, client_error.as_ref().to_owned()).into_response()
    });

    let mut response = err_response.unwrap_or(res);

    let uuid = Uuid::new_v4().to_string();
    let request_id = uuid.parse().expect("can't create request id");

    response.headers_mut().insert("x-request-id", request_id);

    response
}
