pub mod user;
pub mod worker;

use std::sync::Arc;

use rusqlite::Connection;
use tokio::sync::Mutex;

pub type ThreadSafeDatabase = Arc<Mutex<Database>>;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(in_memory: bool, uri: Option<String>) -> anyhow::Result<ThreadSafeDatabase> {
        let conn = if in_memory {
            Connection::open_in_memory()?
        } else {
            Connection::open(uri.ok_or(anyhow::anyhow!("no uri given"))?)?
        };

        let database = Self { conn };

        database.create_tables();

        Ok(Arc::new(Mutex::new(database)))
    }

    pub fn create_tables(&self) {
        user::User::create_table(self);
        worker::Worker::create_table(self);
    }
}

mod tests {
    use super::*;

    #[tokio::test]
    async fn test_creation_of_tables() -> anyhow::Result<()> {
        Database::new(true, None)?;
        Ok(())
    }
}
