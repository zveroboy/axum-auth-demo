use futures::Future;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::error::{Error, Result};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Ticket {
    pub id: u32,
    pub creator_id: u32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTicket {
    pub creator_id: u32,
    pub title: String,
}

pub trait TicketRepository: Sync + Send + Clone {
    fn add(&self, ticket: CreateTicket) -> impl Future<Output = Result<i64>> + Send;
}

pub trait TicketService: Sync + Send + Clone {
    fn create_ticket(&mut self, ticket: CreateTicket) -> impl Future<Output = Result<i64>> + Send;

    fn list_tickets(&self) -> impl Future<Output = Result<Vec<Ticket>>> + Send;

    fn delete_ticket(&mut self, id: u32) -> impl Future<Output = Result<()>> + Send;
}

#[derive(Clone)]
pub struct BaseTicketService<TR> {
    ticket_store: Arc<Mutex<Vec<Ticket>>>,
    ticket_repository: TR,
}

impl<TR> BaseTicketService<TR>
where
    TR: TicketRepository,
{
    pub fn new(ticket_repository: TR) -> Self {
        Self {
            ticket_store: Arc::default(),
            ticket_repository,
        }
    }
}

impl<TR> TicketService for BaseTicketService<TR>
where
    TR: TicketRepository,
{
    async fn create_ticket(&mut self, ticket: CreateTicket) -> Result<i64> {
        Ok(self.ticket_repository.add(ticket).await?)
    }

    async fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().await;

        let tickets = store.clone();

        Ok(tickets)
    }

    async fn delete_ticket(&mut self, id: u32) -> Result<()> {
        let mut store = self.ticket_store.lock().await;

        let index_to_delete = store
            .iter()
            .position(|t| t.id == id)
            .ok_or(Error::EntityNotFound { id: id.to_string() })?;

        store.remove(index_to_delete);

        Ok(())
    }
}
