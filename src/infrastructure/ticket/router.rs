use crate::domain::errors::Result;
use crate::domain::model::{CreateTicket, Ticket, TicketService};
use crate::infrastructure::auth::context::ctx::UserCtx;
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Json, Router};
use tracing::info;

use super::dto::TicketDto;

#[derive(Clone, FromRef)]
pub struct TicketAppState {
    pub ticket_service: TicketService,
}

async fn handle_create_ticket(
    UserCtx { user_id }: UserCtx,
    State(mut ticket_service): State<TicketService>,
    Json(dto): Json<TicketDto>,
) -> Result<Json<Ticket>> {
    let ticket = ticket_service.create_ticket(CreateTicket {
        creator_id: user_id,
        title: dto.title
    }).await;
    info!("ticket added {:?}", ticket);
    ticket.map(|t| Json(t))
}

async fn handle_list_tickets(
    _user_ctx: UserCtx,
    State(ticket_service): State<TicketService>,
) -> Result<Json<Vec<Ticket>>> {
    let tickets = ticket_service.list_tickets().await;
    tickets.map(|t| Json(t))
}

async fn handle_delete_ticket(
    _user_ctx: UserCtx,
    State(mut ticket_service): State<TicketService>,
    Path(id): Path<String>,
) -> StatusCode {
    let _ = ticket_service
        .delete_ticket(id.parse().unwrap_or_default())
        .await;
    StatusCode::OK
}

pub fn create_ticket_router() -> Router<TicketAppState> {
    Router::new()
        .route("/", post(handle_create_ticket).get(handle_list_tickets))
        .route("/:id", delete(handle_delete_ticket))
}
