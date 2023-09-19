use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

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

#[derive(Clone)]
pub struct TicketService {
    ticket_store: Arc<Mutex<Vec<Ticket>>>,
}

impl TicketService {
    pub fn new() -> Self {
        TicketService {
            ticket_store: Arc::default(),
        }
    }
}

impl TicketService {
    pub async fn create_ticket(
        &mut self,
        CreateTicket { title, creator_id }: CreateTicket,
    ) -> super::errors::Result<Ticket> {
        let mut store = self.ticket_store.lock().await;

        let ticket = Ticket {
            id: store.len() as u32 + 1,
            title,
            creator_id,
        };

        store.push(ticket.clone());

        Ok(ticket)
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
