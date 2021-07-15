use crate::models::{Database, ThreadSafeDatabase};
use async_graphql::*;
use serde::{Deserialize, Serialize};

const CREATE_WORKER_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS workers
(
    name          TEXT PRIMARY KEY    NOT NULL,
    readme        TEXT                NOT NULL,
    code          TEXT                NOT NULL,
    frontend_code TEXT                NOT NULL,
    enabled       BOOLEAN             NOT NULL,
    author        TEXT                NOT NULL,
    email         TEXT                NOT NULL,
    url           TEXT                NOT NULL
);";

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Worker {
    pub name: String,
    pub readme: String,
    pub code: String,
    pub frontend_code: String,
    pub enabled: bool,
    pub author: String,
    pub email: String,
    pub url: String,
}

#[Object]
impl Worker {
    pub async fn id(&self) -> ID {
        base64::encode(format!("Worker:{}", self.name).as_bytes()).into()
    }

    pub async fn readme(&self) -> String {
        self.readme.clone()
    }

    pub async fn code(&self) -> String {
        self.code.clone()
    }

    pub async fn frontend_code(&self) -> String {
        self.code.clone()
    }

    pub async fn enabled(&self) -> bool {
        self.enabled
    }

    pub async fn author(&self) -> String {
        self.author.clone()
    }

    pub async fn email(&self) -> String {
        self.email.clone()
    }

    pub async fn url(&self) -> String {
        self.url.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, InputObject)]
pub struct CreateWorkerParams {
    pub name: String,
    pub readme: String,
    pub code: String,
    pub frontend_code: String,
    pub enabled: bool,
    pub author: String,
    pub email: String,
    pub url: String,
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
            "INSERT OR REPLACE INTO workers (name, readme, code, frontend_code, enabled, author, email, url)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![params.name, params.readme, params.code, params.frontend_code, params.enabled, params.author, params.email, params.url],
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

    pub async fn get_all_to_load(database: ThreadSafeDatabase) -> anyhow::Result<Vec<Worker>> {
        let database = database.lock().await;
        let mut stmt = database
            .conn
            .prepare("SELECT name, readme, code, frontend_code, author, email, url FROM workers WHERE enabled = 1")?;
        let worker_scripts = stmt.query_map([], |row| {
            Ok(Worker {
                name: row.get(0)?,
                readme: row.get(1)?,
                code: row.get(2)?,
                frontend_code: row.get(3)?,
                enabled: true,
                author: row.get(4)?,
                email: row.get(5)?,
                url: row.get(6)?,
            })
        })?;

        let worker_scripts = worker_scripts.map(|worker| worker.unwrap());

        Ok(worker_scripts.collect())
    }

    pub async fn get(database: ThreadSafeDatabase, name: String) -> anyhow::Result<Worker> {
        let database = database.lock().await;
        let worker = database.conn.query_row(
            "SELECT name, readme, code, frontend_code, enabled, author, email, url FROM workers WHERE name = ?1",
            rusqlite::params![name],
                |row| {
                    Ok(Worker {
                        name: row.get(0)?,
                        readme: row.get(1)?,
                        code: row.get(2)?,
                        frontend_code: row.get(3)?,
                        enabled: row.get(4)?,
                        author: row.get(5)?,
                        email: row.get(6)?,
                        url: row.get(7)?
                    })
                }
        )?;

        Ok(worker)
    }

    pub async fn get_all(database: ThreadSafeDatabase) -> anyhow::Result<Vec<Worker>> {
        let database = database.lock().await;
        let mut stmt = database.conn.prepare(
            "SELECT name, readme, code, frontend_code, enabled, author, email, url FROM workers",
        )?;
        let worker_scripts = stmt.query_map([], |row| {
            Ok(Worker {
                name: row.get(0)?,
                readme: row.get(1)?,
                code: row.get(2)?,
                frontend_code: row.get(3)?,
                enabled: row.get(4)?,
                author: row.get(5)?,
                email: row.get(6)?,
                url: row.get(7)?,
            })
        })?;

        let worker_scripts = worker_scripts.map(|worker| worker.unwrap());

        Ok(worker_scripts.collect())
    }

    pub async fn get_all_paginated(
        database: ThreadSafeDatabase,
        is_inverted: bool,
        limit: usize,
        after: Option<String>,
        before: Option<String>,
    ) -> anyhow::Result<(bool, bool, Vec<Worker>)> {
        if limit > 50 {
            return Err(anyhow::anyhow!("maximum limit is 50"));
        }
        let database = database.lock().await;

        let order_by = if is_inverted { "DESC" } else { "ASC" };

        let mut query_string = String::from(
            "SELECT name, readme, code, frontend_code, enabled, author, email, url FROM workers",
        );
        let params = rusqlite::params![after.clone(), before.clone(), limit + 1];
        let mut _is_first_where = true;
        if after.is_some() || before.is_some() {
            query_string.push_str(" WHERE");
        }
        if after.is_some() {
            if _is_first_where {
                _is_first_where = false;
            } else {
                query_string.push_str(" AND");
            }
            query_string.push_str(" name > ?1");
        };

        if before.is_some() {
            if _is_first_where {
                _is_first_where = false;
            } else {
                query_string.push_str(" AND");
            }
            query_string.push_str(" name < ?2");
        };

        query_string.push_str(format!(" ORDER BY name {}", order_by).as_str());

        query_string.push_str(" LIMIT ?3");

        let mut stmt = database.conn.prepare(query_string.as_str())?;

        let worker_scripts = stmt.query_map(params, |row| {
            Ok(Worker {
                name: row.get(0)?,
                readme: row.get(1)?,
                code: row.get(2)?,
                frontend_code: row.get(3)?,
                enabled: row.get(4)?,
                author: row.get(5)?,
                email: row.get(6)?,
                url: row.get(7)?,
            })
        })?;

        let worker_scripts = worker_scripts.map(|worker| worker.unwrap());

        let mut workers: Vec<Worker> = worker_scripts.collect();

        let mut has_next_page = false;
        if workers.len() == limit + 1 {
            has_next_page = true;
            workers.pop();
        }

        let has_prev_page = (after.is_some() && !is_inverted) || (before.is_some() && is_inverted);

        Ok((has_prev_page, has_next_page, workers))
    }
}

mod tests {
    #[tokio::test]
    async fn test_users_model() -> anyhow::Result<()> {
        let db = super::super::Database::new(true, true, None).await?;

        // Insert Test Data
        super::Worker::create(
            db.clone(),
            super::CreateWorkerParams {
                name: "test".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                enabled: false,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
            },
        )
        .await?;
        super::Worker::create(
            db.clone(),
            super::CreateWorkerParams {
                name: "test1".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                enabled: true,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
            },
        )
        .await?;
        super::Worker::create(
            db.clone(),
            super::CreateWorkerParams {
                name: "test2".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                enabled: true,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
            },
        )
        .await?;
        super::Worker::create(
            db.clone(),
            super::CreateWorkerParams {
                name: "test3".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                enabled: false,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
            },
        )
        .await?;

        // Try Getting
        let _ = super::Worker::get(db.clone(), "test".to_string()).await?;
        let _ = super::Worker::get(db.clone(), "test1".to_string()).await?;

        // Try Getting All to Load
        let workers = super::Worker::get_all_to_load(db.clone()).await?;
        assert_eq!(workers.len(), 2);

        // Try Getting All
        let workers = super::Worker::get_all(db.clone()).await?;
        assert_eq!(workers.len(), 4);

        // Try Delete One
        let _ = super::Worker::delete(db.clone(), "test".to_string()).await?;
        let workers = super::Worker::get_all(db.clone()).await?;
        assert_eq!(workers.len(), 3);

        // Try Pagination
        let (has_prev_page, has_next_page, workers) =
            super::Worker::get_all_paginated(db.clone(), false, 10, None, None).await?;
        assert_eq!(has_prev_page, false);
        assert_eq!(has_next_page, false);
        assert_eq!(workers.len(), 3);

        let (has_prev_page, has_next_page, workers) =
            super::Worker::get_all_paginated(db.clone(), false, 2, None, None).await?;
        assert_eq!(has_prev_page, false);
        assert_eq!(has_next_page, true);
        assert_eq!(workers.len(), 2);

        let (has_prev_page, has_next_page, workers) = super::Worker::get_all_paginated(
            db.clone(),
            false,
            10,
            Some("test1".to_string()),
            None,
        )
        .await?;
        assert_eq!(has_prev_page, true);
        assert_eq!(has_next_page, false);
        assert_eq!(workers.len(), 2);

        let (has_prev_page, has_next_page, workers) = super::Worker::get_all_paginated(
            db.clone(),
            false,
            10,
            Some("test1".to_string()),
            Some("test2".to_string()),
        )
        .await?;
        assert_eq!(has_prev_page, true);
        assert_eq!(has_next_page, false);
        assert_eq!(workers.len(), 0);

        let (has_prev_page, has_next_page, workers) = super::Worker::get_all_paginated(
            db.clone(),
            false,
            10,
            None,
            Some("test2".to_string()),
        )
        .await?;
        assert_eq!(has_prev_page, false);
        assert_eq!(has_next_page, false);
        assert_eq!(workers.len(), 1);

        Ok(())
    }
}
