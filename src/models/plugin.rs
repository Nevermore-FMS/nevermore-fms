use crate::models::{Database, ThreadSafeDatabase};
use async_graphql::*;
use serde::{Deserialize, Serialize};

const CREATE_WORKER_TABLE: &'static str = "CREATE TABLE IF NOT EXISTS plugins
(
    name          TEXT PRIMARY KEY    NOT NULL,
    readme        TEXT                NOT NULL,
    code          TEXT                NOT NULL,
    frontend_code TEXT                NOT NULL,
    has_frontend  BOOLEAN             NOT NULL,
    enabled       BOOLEAN             NOT NULL,
    author        TEXT                NOT NULL,
    email         TEXT                NOT NULL,
    url           TEXT                NOT NULL,
    plugin_type   TEXT                NOT NULL
);";

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
pub enum PluginType {
    Game,
    NetworkConfigurator,
    Generic,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Plugin {
    pub name: String,
    pub readme: String,
    pub code: String,
    pub frontend_code: String,
    pub has_frontend: bool,
    pub enabled: bool,
    pub author: String,
    pub email: String,
    pub url: String,
    pub plugin_type: PluginType,
}

#[Object]
impl Plugin {
    pub async fn id(&self) -> ID {
        base64::encode(format!("Plugin:{}", self.name).as_bytes()).into()
    }

    pub async fn name(&self) -> String {
        self.name.clone()
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

    pub async fn has_frontend(&self) -> bool {
        self.has_frontend
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

    pub async fn plugin_type(&self) -> PluginType {
        self.plugin_type
    }
}

#[derive(Clone, Deserialize, Serialize, InputObject)]
pub struct CreatePluginParams {
    pub name: String,
    pub readme: String,
    pub code: String,
    pub frontend_code: String,
    pub has_frontend: bool,
    pub enabled: bool,
    pub author: String,
    pub email: String,
    pub url: String,
    pub plugin_type: PluginType,
}

impl Plugin {
    pub(crate) fn create_table(database: &Database) -> anyhow::Result<()> {
        database.conn.execute(CREATE_WORKER_TABLE, [])?;
        Ok(())
    }

    pub async fn create(
        database: ThreadSafeDatabase,
        params: CreatePluginParams,
    ) -> anyhow::Result<()> {
        let database = database.lock().await;
        database.conn.execute(
            "INSERT OR REPLACE INTO plugins (name, readme, code, frontend_code, has_frontend, enabled, author, email, url, plugin_type)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![params.name, params.readme, params.code, params.frontend_code, params.has_frontend, params.enabled, params.author, params.email, params.url, params.plugin_type.to_value().to_string()],
        )?;
        Ok(())
    }

    pub async fn delete(database: ThreadSafeDatabase, name: String) -> anyhow::Result<()> {
        let database = database.lock().await;
        database.conn.execute(
            "DELETE FROM plugins WHERE name = ?1",
            rusqlite::params![name],
        )?;
        Ok(())
    }

    pub async fn get_all_to_load(database: ThreadSafeDatabase) -> anyhow::Result<Vec<Plugin>> {
        let database = database.lock().await;
        let mut stmt = database
            .conn
            .prepare("SELECT name, readme, code, frontend_code, has_frontend, author, email, url, plugin_type FROM plugins WHERE enabled = 1")?;
        let plugin_scripts = stmt.query_map([], |row| {
            Ok(Plugin {
                name: row.get(0)?,
                readme: row.get(1)?,
                code: row.get(2)?,
                frontend_code: row.get(3)?,
                has_frontend: row.get(4)?,
                enabled: true,
                author: row.get(5)?,
                email: row.get(6)?,
                url: row.get(7)?,
                plugin_type: PluginType::parse(Some(Value::String(row.get(8)?))).unwrap(),
            })
        })?;

        let plugin_scripts = plugin_scripts.map(|plugin| plugin.unwrap());

        Ok(plugin_scripts.collect())
    }

    pub async fn get(database: ThreadSafeDatabase, name: String) -> anyhow::Result<Plugin> {
        let database = database.lock().await;
        let plugin = database.conn.query_row(
            "SELECT name, readme, code, frontend_code, has_frontend, enabled, author, email, url, plugin_type FROM plugins WHERE name = ?1",
            rusqlite::params![name],
                |row| {
                    Ok(Plugin {
                        name: row.get(0)?,
                        readme: row.get(1)?,
                        code: row.get(2)?,
                        frontend_code: row.get(3)?,
                        has_frontend: row.get(4)?,
                        enabled: row.get(5)?,
                        author: row.get(6)?,
                        email: row.get(7)?,
                        url: row.get(8)?,
                        plugin_type: PluginType::parse(Some(Value::String(row.get(9)?))).unwrap()
                    })
                }
        )?;

        Ok(plugin)
    }

    pub async fn get_all(database: ThreadSafeDatabase) -> anyhow::Result<Vec<Plugin>> {
        let database = database.lock().await;
        let mut stmt = database.conn.prepare(
            "SELECT name, readme, code, frontend_code, has_frontend, enabled, author, email, url, plugin_type FROM plugins",
        )?;
        let plugin_scripts = stmt.query_map([], |row| {
            Ok(Plugin {
                name: row.get(0)?,
                readme: row.get(1)?,
                code: row.get(2)?,
                frontend_code: row.get(3)?,
                has_frontend: row.get(4)?,
                enabled: row.get(5)?,
                author: row.get(6)?,
                email: row.get(7)?,
                url: row.get(8)?,
                plugin_type: PluginType::parse(Some(Value::String(row.get(9)?))).unwrap(),
            })
        })?;

        let plugin_scripts = plugin_scripts.map(|plugin| plugin.unwrap());

        Ok(plugin_scripts.collect())
    }

    pub async fn get_all_paginated(
        database: ThreadSafeDatabase,
        is_inverted: bool,
        limit: usize,
        after: Option<String>,
        before: Option<String>,
    ) -> anyhow::Result<(bool, bool, Vec<Plugin>)> {
        if limit > 50 {
            return Err(anyhow::anyhow!("maximum limit is 50"));
        }
        let database = database.lock().await;

        let order_by = if is_inverted { "DESC" } else { "ASC" };

        let mut query_string = String::from(
            "SELECT name, readme, code, frontend_code, has_frontend, enabled, author, email, url, plugin_type FROM plugins",
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

        let plugin_scripts = stmt.query_map(params, |row| {
            Ok(Plugin {
                name: row.get(0)?,
                readme: row.get(1)?,
                code: row.get(2)?,
                frontend_code: row.get(3)?,
                has_frontend: row.get(4)?,
                enabled: row.get(5)?,
                author: row.get(6)?,
                email: row.get(7)?,
                url: row.get(8)?,
                plugin_type: PluginType::parse(Some(Value::String(row.get(9)?))).unwrap(),
            })
        })?;

        let plugin_scripts = plugin_scripts.map(|plugin| plugin.unwrap());

        let mut plugins: Vec<Plugin> = plugin_scripts.collect();

        let mut has_next_page = false;
        if plugins.len() == limit + 1 {
            has_next_page = true;
            plugins.pop();
        }

        let has_prev_page = (after.is_some() && !is_inverted) || (before.is_some() && is_inverted);

        Ok((has_prev_page, has_next_page, plugins))
    }
}

mod tests {
    #[tokio::test]
    async fn test_plugins_model() -> anyhow::Result<()> {
        let db = super::super::Database::new(true, true, None).await?;

        // Insert Test Data
        super::Plugin::create(
            db.clone(),
            super::CreatePluginParams {
                name: "test".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                has_frontend: false,
                enabled: false,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
                plugin_type: super::PluginType::Game,
            },
        )
        .await?;
        super::Plugin::create(
            db.clone(),
            super::CreatePluginParams {
                name: "test1".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                has_frontend: false,
                enabled: true,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
                plugin_type: super::PluginType::Game,
            },
        )
        .await?;
        super::Plugin::create(
            db.clone(),
            super::CreatePluginParams {
                name: "test2".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                has_frontend: false,
                enabled: true,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
                plugin_type: super::PluginType::Game,
            },
        )
        .await?;
        super::Plugin::create(
            db.clone(),
            super::CreatePluginParams {
                name: "test3".to_string(),
                readme: "test".to_string(),
                code: "test".to_string(),
                frontend_code: "test".to_string(),
                has_frontend: false,
                enabled: false,
                author: "test".to_string(),
                email: "test".to_string(),
                url: "test".to_string(),
                plugin_type: super::PluginType::Game,
            },
        )
        .await?;

        // Try Getting
        let _ = super::Plugin::get(db.clone(), "test".to_string()).await?;
        let _ = super::Plugin::get(db.clone(), "test1".to_string()).await?;

        // Try Getting All to Load
        let plugins = super::Plugin::get_all_to_load(db.clone()).await?;
        assert_eq!(plugins.len(), 2);

        // Try Getting All
        let plugins = super::Plugin::get_all(db.clone()).await?;
        assert_eq!(plugins.len(), 4);

        // Try Delete One
        let _ = super::Plugin::delete(db.clone(), "test".to_string()).await?;
        let plugins = super::Plugin::get_all(db.clone()).await?;
        assert_eq!(plugins.len(), 3);

        // Try Pagination
        let (has_prev_page, has_next_page, plugins) =
            super::Plugin::get_all_paginated(db.clone(), false, 10, None, None).await?;
        assert_eq!(has_prev_page, false);
        assert_eq!(has_next_page, false);
        assert_eq!(plugins.len(), 3);

        let (has_prev_page, has_next_page, plugins) =
            super::Plugin::get_all_paginated(db.clone(), false, 2, None, None).await?;
        assert_eq!(has_prev_page, false);
        assert_eq!(has_next_page, true);
        assert_eq!(plugins.len(), 2);

        let (has_prev_page, has_next_page, plugins) = super::Plugin::get_all_paginated(
            db.clone(),
            false,
            10,
            Some("test1".to_string()),
            None,
        )
        .await?;
        assert_eq!(has_prev_page, true);
        assert_eq!(has_next_page, false);
        assert_eq!(plugins.len(), 2);

        let (has_prev_page, has_next_page, plugins) = super::Plugin::get_all_paginated(
            db.clone(),
            false,
            10,
            Some("test1".to_string()),
            Some("test2".to_string()),
        )
        .await?;
        assert_eq!(has_prev_page, true);
        assert_eq!(has_next_page, false);
        assert_eq!(plugins.len(), 0);

        let (has_prev_page, has_next_page, plugins) = super::Plugin::get_all_paginated(
            db.clone(),
            false,
            10,
            None,
            Some("test2".to_string()),
        )
        .await?;
        assert_eq!(has_prev_page, false);
        assert_eq!(has_next_page, false);
        assert_eq!(plugins.len(), 1);

        Ok(())
    }
}
