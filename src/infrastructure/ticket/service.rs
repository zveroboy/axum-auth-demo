use crate::domain::ticket::ticket::{BaseTicketService, CreateTicket, TicketRepository};
use crate::infrastructure::store::Db;
use std::convert::Infallible;
use std::pin::Pin;

use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use futures::Future;
use sqlx::PgPool;

use crate::domain;

#[derive(Clone)]
pub struct SqlxTicketRepository {
    db: Db,
}

impl SqlxTicketRepository {
    pub fn new(db: Db) -> Self {
        SqlxTicketRepository { db }
    }
}

impl TicketRepository for SqlxTicketRepository {
    async fn add(&self, ticket: CreateTicket) -> domain::error::Result<i64> {
        let (id,) = sqlx::query_as::<_, (i64,)>("INSERT INTO task(title) VALUES ($1) RETURNING id")
            .bind(ticket.title)
            .fetch_one(&self.db)
            .await
            .unwrap();

        Ok(id)
        // sqlx::query("INSERT INTO task(title) VALUES ($1)").fetch(self.db).await?
    }
}

pub type RestTicketService = BaseTicketService<SqlxTicketRepository>;

#[derive(Clone, FromRef)]
pub struct BaseTicketAppState {
    pub ticket_service: RestTicketService,
}

impl<S: Send + Sync> FromRequestParts<S> for BaseTicketAppState
where
    PgPool: FromRef<S>,
{
    type Rejection = Infallible;

    fn from_request_parts<'a, 'b, 'at>(
        _parts: &'a mut Parts,
        state: &'b S,
    ) -> Pin<Box<(dyn Future<Output = Result<BaseTicketAppState, Self::Rejection>> + Send + 'at)>>
    where
        'a: 'at,
        'b: 'at,
        Self: 'at,
    {
        Box::pin(async {
            let pool = PgPool::from_ref(state);
            let ticket_service = BaseTicketService::new(SqlxTicketRepository::new(pool.clone()));

            Ok(BaseTicketAppState { ticket_service })
        })
    }
}
