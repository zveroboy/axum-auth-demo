use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::errors::Result;

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

#[async_trait]
pub trait TicketRepository: Sync + Send {
    async fn add(&self, ticket: CreateTicket) -> Result<i64>;
}

#[derive(Clone)]
pub struct TicketService {
    ticket_store: Arc<Mutex<Vec<Ticket>>>,
    ticket_repository: Arc<dyn TicketRepository>,
}

impl TicketService {
    pub fn new(ticket_repository: Arc<dyn TicketRepository>) -> Self {
        TicketService {
            ticket_store: Arc::default(),
            ticket_repository: ticket_repository,
        }
    }
}

impl TicketService {
    pub async fn create_ticket(&mut self, ticket: CreateTicket) -> super::errors::Result<i64> {
        // let mut store = self.ticket_store.lock().await;

        // let ticket = Ticket {
        //     title,
        //     creator_id,
        // };

        Ok(self.ticket_repository.add(ticket).await?)

        // store.push(ticket.clone());

        // Ok(0)
    }

    pub async fn list_tickets(&self) -> super::errors::Result<Vec<Ticket>> {
        let store = self.ticket_store.lock().await;

        let tickets = store.clone();

        Ok(tickets)
    }

    pub async fn delete_ticket(&mut self, id: u32) -> super::errors::Result<()> {
        let mut store = self.ticket_store.lock().await;

        let index_to_delete = store
            .iter()
            .position(|t| t.id == id)
            .ok_or(super::errors::Error::EntityNotFound { id: id.to_string() })?;

        store.remove(index_to_delete);

        Ok(())
    }
}
