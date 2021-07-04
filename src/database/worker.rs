use crate::database::{Database, ThreadSafeDatabase};
use async_graphql::*;
use serde::{Deserialize, Serialize};

const CREATE_WORKER_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS workers
(
    name        TEXT PRIMARY KEY    NOT NULL,
    description TEXT                NOT NULL,
    code        TEXT                NOT NULL,
    enabled     BOOLEAN             NOT NULL
);";

#[derive(Clone, Deserialize, Serialize, SimpleObject)]
pub struct Worker {
    pub name: String,
    pub description: String,
    pub code: String,
    pub enabled: bool,
}

#[derive(Clone, Deserialize, Serialize, InputObject)]
pub struct CreateWorkerParams {
    pub name: String,
    pub description: String,
    pub code: String,
    pub enabled: bool,
}

impl Worker {
    pub(crate) fn create_table(database: &Database) -> anyhow::Result<()> {
        database.conn.execute(CREATE_WORKER_TABLE, [])?;
        Ok(())
    }

    pub async fn create(
        database: ThreadSafeDatabase,
        params: CreateWorkerParams,
    ) -> anyhow::Result<()> {
        let database = database.lock().await;
        database.conn.execute(
            "INSERT OR REPLACE INTO workers (name, description, code, enabled)
         VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![params.name, params.description, params.code, params.enabled],
        )?;
        Ok(())
    }

    pub async fn delete(database: ThreadSafeDatabase, name: String) -> anyhow::Result<()> {
        let database = database.lock().await;
        database.conn.execute(
            "DELETE FROM workers WHERE name = ?1",
            rusqlite::params![name],
        )?;
        Ok(())
    }

    pub async fn get_all_workers_to_load(
        database: ThreadSafeDatabase,
    ) -> anyhow::Result<Vec<Worker>> {
        let database = database.lock().await;
        let mut stmt = database
            .conn
            .prepare("SELECT name, description, code FROM workers WHERE enabled = 1")?;
        let worker_scripts = stmt.query_map([], |row| {
            Ok(Worker {
                name: row.get(0)?,
                description: row.get(1)?,
                code: row.get(2)?,
                enabled: true,
            })
        })?;

        let worker_scripts = worker_scripts.map(|worker| worker.unwrap());

        Ok(worker_scripts.collect())
    }

    pub async fn get_all_workers(database: ThreadSafeDatabase) -> anyhow::Result<Vec<Worker>> {
        let database = database.lock().await;
        let mut stmt = database
            .conn
            .prepare("SELECT name, description, code, enabled FROM workers")?;
        let worker_scripts = stmt.query_map([], |row| {
            Ok(Worker {
                name: row.get(0)?,
                description: row.get(1)?,
                code: row.get(2)?,
                enabled: row.get(3)?,
            })
        })?;

        let worker_scripts = worker_scripts.map(|worker| worker.unwrap());

        Ok(worker_scripts.collect())
    }
}
