use std::fmt::Debug;

use tracing::debug;

use crate::domain::user::entity::User;
use crate::domain::user::error::UserResult;
use crate::domain::user::repository::{CreateParams, UserRepository};
use crate::infrastructure::store::Db;

#[derive(Clone)]
pub struct SqlxUserRepository {
    db: Db,
}

impl SqlxUserRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl UserRepository for SqlxUserRepository {
    async fn create(&self, CreateParams { email, password }: CreateParams) -> UserResult<i64> {
        debug!("create: {email}, {password}");
        let (id,) = sqlx::query_as::<_, (i64,)>(
            "INSERT INTO \"user\"(email, password) VALUES ($1, $2) RETURNING id",
        )
        .bind(email)
        .bind(password)
        .fetch_one(&self.db)
        .await
        .unwrap(); // TODO: handle sqlx error

        println!("created: {id}");
        Ok(id)
    }

    async fn find_by_email<P: AsRef<str> + Sync + Send + Debug>(
        &self,
        email: P,
    ) -> UserResult<User> {
        println!("find_by_email: {:?}", &email);
        let user: User = sqlx::query_as("SELECT * FROM \"user\" WHERE email=$1 LIMIT 1")
            .bind(email.as_ref())
            .fetch_one(&self.db)
            .await
            .unwrap(); // TODO: handle sqlx error, e.g. Err::RowNotFound

        Ok(user)
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
