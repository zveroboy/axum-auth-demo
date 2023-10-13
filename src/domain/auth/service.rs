pub struct LoginParams {
    email: String,
    password: String,
}

#[async_trait]
pub trait AuthRepository: Sync + Send + Clone {
    async fn login(&self, params: LoginParams) -> Result<i64>;
}

#[async_trait]
pub trait AuthService {
    type Repository;

    async fn create_ticket(&mut self, ticket: CreateTicket) -> Result<i64>;

    async fn list_tickets(&self) -> Result<Vec<Ticket>>;

    async fn delete_ticket(&mut self, id: u32) -> Result<()>;
}

#[derive(Clone)]
pub struct BaseTicketService<TR> {
    ticket_store: Arc<Mutex<Vec<Ticket>>>,
    ticket_repository: TR,
}

impl<TR: TicketRepository> BaseTicketService<TR> {
    pub fn new(ticket_repository: TR) -> Self {
        Self {
            ticket_store: Arc::default(),
            ticket_repository,
        }
    }
}
