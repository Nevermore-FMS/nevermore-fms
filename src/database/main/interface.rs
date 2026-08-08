use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};

#[derive(Clone)]
pub struct MainDbInterface {
    pub(super) db_pool: Pool<ConnectionManager<SqliteConnection>>
}

impl MainDbInterface {
    pub fn new(db_pool: Pool<ConnectionManager<SqliteConnection>>) -> Self {
        MainDbInterface { db_pool }
    }
}