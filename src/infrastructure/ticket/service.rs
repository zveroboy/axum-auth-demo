use crate::domain::error::Result;
use crate::domain::ticket::ticket::{CreateTicket, TicketRepository};
use crate::infrastructure::store::Db;
use async_trait::async_trait;

#[derive(Clone)]
pub struct PgTicketRepository {
    db: Db,
}

impl PgTicketRepository {
    pub fn new(db: Db) -> Self {
        PgTicketRepository { db }
    }
}

#[async_trait]
impl TicketRepository for PgTicketRepository {
    async fn add(&self, ticket: CreateTicket) -> Result<i64> {
        let (id,) = sqlx::query_as::<_, (i64,)>("INSERT INTO task(title) VALUES ($1) RETURNING id")
            .bind(ticket.title)
            .fetch_one(&self.db)
            .await
            .unwrap();

        Ok(id)
        // sqlx::query("INSERT INTO task(title) VALUES ($1)").fetch(self.db).await?
    }
}

// #[derive(Clone)]
// pub struct TicketService {
//     ticket_store: Arc<Mutex<Vec<Ticket>>>,
//     ticket_repository: dyn TicketRepository,
// }

// impl TicketService {
//     pub fn new(ticket_repository: dyn TicketRepository) -> Self {
//         TicketService {
//             ticket_store: Arc::default(),
//             ticket_repository
//         }
//     }
// }

// impl TicketService {
//     pub async fn create_ticket(
//         &mut self,
//         CreateTicket { title, creator_id }: CreateTicket,
//     ) -> Result<Ticket> {
//         let mut store = self.ticket_store.lock().await;

//         let ticket = Ticket {
//             id: store.len() as u32 + 1,
//             title,
//             creator_id,
//         };

//         store.push(ticket.clone());

//         Ok(ticket)
//     }

//     pub async fn list_tickets(&self) -> Result<Vec<Ticket>> {
//         let store = self.ticket_store.lock().await;

//         let tickets = store.clone();

//         Ok(tickets)
//     }

//     pub async fn delete_ticket(&mut self, id: u32) -> Result<()> {
//         let mut store = self.ticket_store.lock().await;

//         let index_to_delete = store
//             .iter()
//             .position(|t| t.id == id)
//             .ok_or(Error::EntityNotFound { id: id.to_string() })?;

//         store.remove(index_to_delete);

//         Ok(())
//     }
// }
