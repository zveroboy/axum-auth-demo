use crate::domain::ticket::ticket::BaseTicketService;

use super::handlers::{handle_create_ticket, handle_delete_ticket, handle_list_tickets};
use super::service::PgTicketRepository;
use axum::extract::FromRef;
use axum::routing::{delete, post};
use axum::Router;

#[derive(Clone, FromRef)]
pub struct BaseTicketAppState {
    pub ticket_service: BaseTicketService<PgTicketRepository>,
}

pub fn ticket_router() -> Router<BaseTicketAppState> {
    type TicketService = BaseTicketService<PgTicketRepository>;

    Router::new()
        .route(
            "/",
            post(handle_create_ticket::<TicketService>).get(handle_list_tickets::<TicketService>),
        )
        .route("/:id", delete(handle_delete_ticket::<TicketService>))
}
