use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct TicketDto {
    pub title: String,
}
