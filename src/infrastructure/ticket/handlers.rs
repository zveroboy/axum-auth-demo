use crate::domain::ticket::ticket::{CreateTicket, Ticket, TicketService};
use crate::infrastructure::context::ctx::UserCtx;
use crate::infrastructure::middleware::error::AppError;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use tracing::{debug, info};

use super::dto::TicketDto;

pub async fn handle_create_ticket<TS: TicketService>(
    UserCtx { user_id }: UserCtx,
    State(mut ticket_service): State<TS>,
    Json(dto): Json<TicketDto>,
) -> Result<Json<i64>, AppError> {
    let ticket_id = ticket_service
        .create_ticket(CreateTicket {
            creator_id: user_id,
            title: dto.title,
        })
        .await?;
    info!("ticket added {:?}", ticket_id);
    Ok(ticket_id.into())
}

pub async fn handle_list_tickets<TS: TicketService>(
    _user_ctx: UserCtx,
    State(ticket_service): State<TS>,
) -> Result<Json<Vec<Ticket>>, AppError> {
    debug!(user_id = _user_ctx.user_id);
    let tickets = ticket_service.list_tickets().await?;
    Ok(tickets.into())
}

pub async fn handle_delete_ticket<TS: TicketService>(
    _user_ctx: UserCtx,
    State(mut ticket_service): State<TS>,
    Path(id): Path<String>,
) -> StatusCode {
    let _ = ticket_service
        .delete_ticket(id.parse().unwrap_or_default())
        .await;
    StatusCode::OK
}
