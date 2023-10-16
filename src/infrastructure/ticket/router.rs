use crate::domain::ticket::ticket::TicketService;
use crate::infrastructure::auth::service::PgUserRepository;

use super::handlers::{handle_create_ticket, handle_delete_ticket, handle_list_tickets};
use axum::routing::{delete, post};
use axum::Router;

// // TODO: Fix implementation details leaking
// #[derive(Clone)]
// pub struct BaseTicketAppState<TR> {
//     pub ticket_service: BaseTicketService<TR>,
// }

// impl<TR: Clone> FromRef<BaseTicketAppState<TR>> for BaseTicketService<TR> {
//     fn from_ref(state: &BaseTicketAppState<TR>) -> BaseTicketService<TR> {
//         state.ticket_service.clone()
//     }
// }

pub fn ticket_router<TS>() -> Router<TS>
where
    TS: TicketService + Send + Sync + 'static,
{
    Router::new()
        .route(
            "/",
            post(handle_create_ticket::<TS>).get(handle_list_tickets::<TS>),
        )
        .route("/:id", delete(handle_delete_ticket::<TS>))
}
