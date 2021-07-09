use crate::database::{Database, ThreadSafeDatabase};
use async_graphql::*;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

const CREATE_USER_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS users
(
    full_name     TEXT             NOT NULL,
    username      TEXT             NOT NULL,
    password      TEXT             NOT NULL,
    pin           TEXT             NOT NULL
);";

#[derive(Clone, Debug)]
pub struct User {
    pub row_id: i32,
    pub full_name: String,
    pub username: String,
    pub password: String,
    pub pin: String,
}

#[Object]
impl User {
    pub async fn id(&self) -> String {
        base64::encode(self.row_id.to_string().as_bytes())
    }

    pub async fn full_name(&self) -> String {
        self.full_name.clone()
    }

    pub async fn username(&self) -> String {
        self.username.clone()
    }
}

#[derive(Clone, Deserialize, Serialize, InputObject)]
pub struct CreateUserParams {
    pub full_name: String,
    pub username: String,
    pub password: String,
    pub pin: String,
}

impl User {
    pub(crate) fn create_table(database: &Database) -> anyhow::Result<()> {
        database.conn.execute(CREATE_USER_TABLE, [])?;
        Ok(())
    }

    pub async fn create(
        database: ThreadSafeDatabase,
        params: CreateUserParams,
    ) -> anyhow::Result<()> {
        let database = database.lock().await;
        let config = argon2::Config::default();
        let password = argon2::hash_encoded(params.password.as_bytes(), "test".as_bytes(), &config)?;
        let pin = argon2::hash_encoded(params.pin.as_bytes(), "test".as_bytes(), &config)?;
        database.conn.execute(
            "INSERT OR REPLACE INTO users (full_name, username, password, pin)
         VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![params.full_name, params.username, password, pin],
        )?;
        Ok(())
    }

    pub async fn delete(database: ThreadSafeDatabase, row_id: i32) -> anyhow::Result<()> {
        let database = database.lock().await;
        database.conn.execute(
            "DELETE FROM users WHERE rowid = ?1",
            rusqlite::params![row_id],
        )?;
        Ok(())
    }

    pub async fn get(database: ThreadSafeDatabase, row_id: i32) -> anyhow::Result<User> {
        let database = database.lock().await;
        let user = database.conn.query_row(
            "SELECT rowid, full_name, username, password, pin FROM users WHERE rowid = ?1",
            rusqlite::params![row_id],
                |row| {
                    Ok(User {
                        row_id: row.get(1)?,
                        full_name: row.get(2)?,
                        username: row.get(3)?,
                        password: "".to_string(),
                        pin: "".to_string(),
                    })
                }
        )?;

        Ok(user)
    }



    pub async fn get_all_paginated(database: ThreadSafeDatabase, is_inverted: bool, limit: usize, after: Option<String>, before: Option<String>) -> anyhow::Result<(bool, bool, Vec<User>)> {
        if limit > 50 {
            return Err(anyhow::anyhow!("maximum limit is 50"))
        }
        let database = database.lock().await;

        let order_by = if is_inverted {
            "DESC"
        } else {
            "ASC"
        };

        let mut query_string = String::from("SELECT rowid, full_name, username FROM users");
        let params = rusqlite::params![after.clone(), before.clone(), limit + 1];
        let mut is_first_where = true;
        if after.is_some() || before.is_some() {
            query_string.push_str(" WHERE");
        }
        if after.is_some() {
            if is_first_where {
                is_first_where = false;
            } else {
                query_string.push_str(" AND");
            }
            query_string.push_str(" rowid > ?1");
        };

        if before.is_some() {
            if is_first_where {
                is_first_where = false;
            } else {
                query_string.push_str(" AND");
            }
            query_string.push_str(" rowid < ?2");
        };

        query_string.push_str(format!(" ORDER BY rowid {}", order_by).as_str());

        query_string.push_str(" LIMIT ?3");

        let mut stmt = database.conn.prepare(query_string.as_str())?;

        let users = stmt.query_map(params, |row| {
            Ok(User {
                row_id: row.get(1)?,
                full_name: row.get(2)?,
                username: row.get(3)?,
                password: "".to_string(),
                pin: "".to_string(),
            })
        })?;

        let users = users.map(|user| user.unwrap());

        let mut users: Vec<User> = users.collect();

        let mut has_next_page = false;
        if users.len() == limit + 1 {
            has_next_page = true;
            users.pop();
        }

        let has_prev_page = (after.is_some() && !is_inverted) || (before.is_some() && is_inverted);

        Ok((has_prev_page, has_next_page, users))
    }
}
