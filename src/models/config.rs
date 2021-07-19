use crate::models::{Database, ThreadSafeDatabase};
use async_graphql::*;

const CREATE_CONFIG_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS config
(
    key      TEXT PRIMARY KEY NOT NULL,
    value    TEXT            NOT NULL
);";

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum ConfigKey {
    HasSetup,
    EventName,                 // The name of the current event.
    ActiveNetworkConfigurator, // The name of the active network configurator.
    ShareCrashAnalytics,
    PrivateKey,
}

#[derive(Clone, Debug)]
pub struct Config {}

impl Config {
    pub(crate) fn create_table(database: &Database) -> anyhow::Result<()> {
        database.conn.execute(CREATE_CONFIG_TABLE, [])?;
        Ok(())
    }

    pub async fn set(
        database: ThreadSafeDatabase,
        key: ConfigKey,
        value: String,
    ) -> anyhow::Result<()> {
        let database = database.lock().await;

        let key = key.to_value().to_string();

        database.conn.execute(
            "INSERT OR REPLACE INTO config (key, value)
         VALUES (?1, ?2)",
            rusqlite::params![key, value],
        )?;
        Ok(())
    }

    pub async fn get(database: ThreadSafeDatabase, key: ConfigKey) -> Option<String> {
        let database = database.lock().await;

        let key = key.to_value().to_string();

        let user = database
            .conn
            .query_row(
                "SELECT value FROM config WHERE key = ?1",
                rusqlite::params![key],
                |row| {
                    let value: String = row.get(0)?;
                    Ok(value)
                },
            )
            .ok();

        user
    }
}

mod tests {
    #[tokio::test]
    async fn test_configuration() -> anyhow::Result<()> {
        let db = super::super::Database::new(true, true, None).await?;

        super::Config::set(db.clone(), super::ConfigKey::EventName, "test".to_string()).await?;
        let value = super::Config::get(db.clone(), super::ConfigKey::EventName)
            .await
            .unwrap();
        assert_eq!(value, "test".to_string());
        Ok(())
    }
}
