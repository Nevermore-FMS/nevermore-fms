use crate::database::Database;

const CREATE_USER_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS todos
(
    id          INTEGER PRIMARY KEY NOT NULL,
    description TEXT                NOT NULL,
    done        BOOLEAN             NOT NULL DEFAULT 0
);";

pub struct User {}

impl User {
    pub(crate) fn create_table(database: &Database) -> anyhow::Result<()> {
        database.conn.execute(CREATE_USER_TABLE, [])?;
        Ok(())
    }
}
