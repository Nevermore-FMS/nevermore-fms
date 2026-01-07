use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use directories::ProjectDirs;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/main");

fn get_db_path(override_default_path: Option<PathBuf>, db_name: &str) -> anyhow::Result<PathBuf> {
    let dir_path = if let Some(custom_path) = override_default_path {
        custom_path
    } else if let Some(proj_dirs) = ProjectDirs::from("com", "edgarallanohms", "Nevermore-FMS") {
        proj_dirs.data_local_dir().to_path_buf()
    } else {
        anyhow::bail!("Unable to establish connection to database using provided path information");
    };

    let db_path = dir_path.join(format!("{db_name}.nvdb"));

    Ok(db_path)
}

pub fn init_main_db_pool(
    override_default_path: Option<PathBuf>,
) -> anyhow::Result<SqliteConnection> {
    let main_db_path = get_db_path(override_default_path, "main")?;

    if let Some(parent_dir) = main_db_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let _ = OpenOptions::new().create(true).append(true).open(main_db_path.clone())?;

    let mut conn = SqliteConnection::establish(
        main_db_path
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid database path provided"))?,
    )?;

    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;

    Ok(conn)
}
