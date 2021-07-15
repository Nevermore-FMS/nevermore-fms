pub mod user;
pub mod worker;

use std::{collections::HashMap, sync::Arc};

use rusqlite::{Connection, Error, Row, ToSql};
use serde_json::Value;
use tokio::sync::Mutex;

pub type ThreadSafeDatabase = Arc<Mutex<Database>>;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub async fn new(
        create_default_tables: bool,
        in_memory: bool,
        uri: Option<String>,
    ) -> anyhow::Result<ThreadSafeDatabase> {
        tokio::task::spawn_blocking(move || {
            let conn = if in_memory {
                Connection::open_in_memory()?
            } else {
                Connection::open(uri.ok_or(anyhow::anyhow!("no uri given"))?)?
            };

            let database = Self { conn };

            if create_default_tables {
                database.create_tables()?;
            };

            Ok(Arc::new(Mutex::new(database)))
        })
        .await?
    }

    pub async fn execute(&self, stmt: String, params: Vec<Value>) -> anyhow::Result<usize> {
        let params = rewrite_params(params);
        Ok(self
            .conn
            .execute(stmt.as_str(), rusqlite::params_from_iter(params.iter()))?)
    }

    pub async fn query_row(
        &self,
        stmt: String,
        params: Vec<Value>,
    ) -> anyhow::Result<HashMap<String, Value>> {
        let params = rewrite_params(params);
        Ok(self.conn.query_row(
            stmt.as_str(),
            rusqlite::params_from_iter(params.iter()),
            |x| Ok(decode_row(x)?),
        )?)
    }

    pub async fn query_rows(
        &self,
        stmt: String,
        params: Vec<Value>,
    ) -> anyhow::Result<Vec<HashMap<String, Value>>> {
        let params = rewrite_params(params);
        let mut stmt = self.conn.prepare_cached(stmt.as_str())?;
        let out = stmt.query_map(rusqlite::params_from_iter(params.iter()), |x| {
            Ok(decode_row(x)?)
        })?;
        let out = out.map(|row| row.unwrap());
        Ok(out.collect())
    }

    /*pub async fn query(&self, stmt: String, params: Vec<Value>) -> anyhow::Result<HashMap<String, Value>> {
        let stmt = self.conn.prepare(stmt.as_str())?;
        stmt.query_map(rusqlite::params_from_iter(params.iter()), |x| {

        })
    }*/

    pub fn create_tables(&self) -> anyhow::Result<()> {
        user::User::create_table(self)?;
        worker::Worker::create_table(self)?;
        Ok(())
    }
}

fn rewrite_params(params: Vec<Value>) -> Vec<Box<dyn ToSql>> {
    let mut new_params: Vec<Box<dyn ToSql>> = Vec::new();
    for param in params {
        let out: Box<dyn ToSql> = match param {
            Value::Null => Box::new(rusqlite::types::Value::Null),
            Value::Bool(value) => Box::new(value),
            Value::Number(value) => {
                if value.is_f64() {
                    Box::new(value.as_f64())
                } else if value.is_i64() {
                    Box::new(value.as_i64())
                } else {
                    Box::new(value.as_u64())
                }
            }
            Value::String(_) => Box::new(param),
            Value::Array(_) => Box::new(param),
            Value::Object(_) => Box::new(param),
        };
        new_params.push(out);
    }
    new_params
}

fn decode_row<'a>(row: &Row<'a>) -> Result<HashMap<String, Value>, Error> {
    let names = row.column_names();
    let mut map: HashMap<String, Value> = HashMap::new();
    for name in names {
        let index = row.column_index(name)?;
        let out = row.get_ref(index)?;
        match out.data_type() {
            rusqlite::types::Type::Real => {
                let num: f64 = row.get(index)?;
                map.insert(name.to_string(), serde_json::json!(num));
            }
            rusqlite::types::Type::Null => {
                map.insert(name.to_string(), serde_json::json!(null));
            }
            rusqlite::types::Type::Integer => {
                let num: i64 = row.get(index)?;
                map.insert(name.to_string(), serde_json::json!(num));
            }
            rusqlite::types::Type::Text => {
                map.insert(name.to_string(), row.get(index)?);
            }
            rusqlite::types::Type::Blob => {
                map.insert(name.to_string(), row.get(index)?);
            }
        }
    }
    Ok(map)
}

mod tests {
    #[tokio::test]
    async fn test_creation_of_tables() -> anyhow::Result<()> {
        super::Database::new(true, true, None).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_execute() -> anyhow::Result<()> {
        let db = super::Database::new(false, true, None).await?;
        let _ = db
            .lock()
            .await
            .execute(
                "CREATE TABLE test (
            test TEXT NOT NULL
         );"
                .to_string(),
                Vec::new(),
            )
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_query_row() -> anyhow::Result<()> {
        let db = super::Database::new(false, true, None).await?;
        let out = db
            .lock()
            .await
            .query_row("SELECT COUNT(*) FROM users".to_string(), Vec::new())
            .await?;
        assert!(out.contains_key("COUNT(*)"));
        Ok(())
    }

    #[tokio::test]
    async fn test_query_row_params() -> anyhow::Result<()> {
        let db = super::Database::new(false, true, None).await?;

        // Numeric Test
        let _ = db
            .clone()
            .lock()
            .await
            .execute(
                "CREATE TABLE tests (
            test TEXT NOT NULL
         );"
                .to_string(),
                Vec::new(),
            )
            .await?;
        let _ = db
            .clone()
            .lock()
            .await
            .execute(
                "INSERT INTO tests (test) VALUES (?1)".to_string(),
                vec![serde_json::json!("test")],
            )
            .await?;
        let out = db
            .clone()
            .lock()
            .await
            .query_row(
                "SELECT rowid, test FROM tests WHERE rowid = ?1".to_string(),
                vec![serde_json::json!(1)],
            )
            .await?;
        assert!(out.contains_key("test"));

        // String Test
        let _ = db
            .clone()
            .lock()
            .await
            .execute(
                "CREATE TABLE test_strings (
            test TEXT PRIMARY KEY
         );"
                .to_string(),
                Vec::new(),
            )
            .await?;
        let _ = db
            .clone()
            .lock()
            .await
            .execute(
                "INSERT INTO test_strings (test) VALUES (?1)".to_string(),
                vec![serde_json::json!("test")],
            )
            .await?;
        let out = db
            .clone()
            .lock()
            .await
            .query_row(
                "SELECT test FROM test_strings WHERE test = ?1".to_string(),
                vec![serde_json::json!("test")],
            )
            .await?;
        assert!(out.contains_key("test"));
        Ok(())
    }
}
