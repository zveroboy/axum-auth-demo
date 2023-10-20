use crate::domain::ticket::ticket::BaseTicketService;

use self::service::SqlxTicketRepository;

pub mod dto;
pub mod handlers;
pub mod router;
pub mod service;