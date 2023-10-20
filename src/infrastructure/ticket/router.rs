use crate::infrastructure::state::AppState;

use super::handlers::{handle_create_ticket, handle_delete_ticket, handle_list_tickets};
use axum::routing::{delete, post};
use axum::Router;

pub fn ticket_router() -> Router<AppState> {
    Router::new()
        .route("/", post(handle_create_ticket).get(handle_list_tickets))
        .route("/:id", delete(handle_delete_ticket))
}
