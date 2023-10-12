use crate::domain::ticket::ticket::{CreateTicket, Ticket, TicketRepository, TicketService};
use crate::infrastructure::context::ctx::UserCtx;
use crate::infrastructure::middleware::error::ClientError;
use axum::extract::{FromRef, Path, State};
use axum::http::StatusCode;
use axum::routing::{delete, post};
use axum::{Json, Router};
use tracing::{debug, info};

use super::dto::TicketDto;

#[derive(Clone)]
pub struct TicketAppState<TR> {
    pub ticket_service: TicketService<TR>,
}

impl<TR: Clone> FromRef<TicketAppState<TR>> for TicketService<TR> {
    fn from_ref(state: &TicketAppState<TR>) -> TicketService<TR> {
        state.ticket_service.clone()
    }
}

async fn handle_create_ticket<TR>(
    UserCtx { user_id }: UserCtx,
    State(mut ticket_service): State<TicketService<TR>>,
    Json(dto): Json<TicketDto>,
) -> Result<Json<i64>, ClientError>
where
    TR: TicketRepository,
{
    let ticket_id = ticket_service
        .create_ticket(CreateTicket {
            creator_id: user_id,
            title: dto.title,
        })
        .await?;
    info!("ticket added {:?}", ticket_id);
    Ok(ticket_id.into())
}

async fn handle_list_tickets<TR>(
    _user_ctx: UserCtx,
    State(ticket_service): State<TicketService<TR>>,
) -> Result<Json<Vec<Ticket>>, ClientError>
where
    TR: TicketRepository,
{
    debug!(user_id = _user_ctx.user_id);
    let tickets = ticket_service.list_tickets().await?;
    Ok(tickets.into())
}

async fn handle_delete_ticket<TR>(
    _user_ctx: UserCtx,
    State(mut ticket_service): State<TicketService<TR>>,
    Path(id): Path<String>,
) -> StatusCode
where
    TR: TicketRepository,
{
    let _ = ticket_service
        .delete_ticket(id.parse().unwrap_or_default())
        .await;
    StatusCode::OK
}

pub fn ticket_router<TR>() -> Router<TicketAppState<TR>>
where
    TR: TicketRepository + 'static,
{
    Router::new()
        .route("/", post(handle_create_ticket).get(handle_list_tickets))
        .route("/:id", delete(handle_delete_ticket))
}
