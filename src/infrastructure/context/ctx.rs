#[allow(dead_code)]
#[derive(Clone)]
pub struct UserCtx {
    pub user_id: u32,
}

impl UserCtx {
    pub fn new(user_id: u32) -> Self {
        UserCtx { user_id }
    }

    fn user_id(&self) -> u32 {
        self.user_id
    }
}
