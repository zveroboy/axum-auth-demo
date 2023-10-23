use crate::domain::ticket::ticket::{CreateTicket, Ticket, TicketService};
use crate::infrastructure::context::ctx::UserCtx;
use crate::infrastructure::middleware::error::AppError;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use tracing::info;

use super::dto::TicketDto;
use super::service::BaseTicketAppState;

pub async fn handle_create_ticket(
    // UserCtx { user_id }: UserCtx,
    BaseTicketAppState { mut ticket_service }: BaseTicketAppState,
    Json(dto): Json<TicketDto>,
) -> Result<Json<i64>, AppError> {
    let ticket_id = ticket_service
        .create_ticket(CreateTicket {
            creator_id: 123,
            title: dto.title,
        })
        .await?;
    info!("ticket added {:?}", ticket_id);
    Ok(ticket_id.into())
}

pub async fn handle_list_tickets(
    // _user_ctx: UserCtx,
    BaseTicketAppState { ticket_service }: BaseTicketAppState,
) -> Result<Json<Vec<Ticket>>, AppError> {
    // debug!(user_id = _user_ctx.user_id);
    let tickets = ticket_service.list_tickets().await?;
    Ok(tickets.into())
}

pub async fn handle_delete_ticket(
    _user_ctx: UserCtx,
    BaseTicketAppState { mut ticket_service }: BaseTicketAppState,
    Path(id): Path<String>,
) -> StatusCode {
    let _ = ticket_service
        .delete_ticket(id.parse().unwrap_or_default())
        .await;
    StatusCode::OK
}
