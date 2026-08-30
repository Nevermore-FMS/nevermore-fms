use std::{
    fs::{self, OpenOptions}, path::Path,
};

use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

use crate::database::main::interface::{MainDbInterface};

const MAIN_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/main");

fn get_db_path(data_path: &Path, db_name: &str) -> std::path::PathBuf {
    data_path.join(format!("{db_name}.nvdb"))
}

fn init_db_pool(
    data_path: &Path,
    db_name: &str,
) -> anyhow::Result<Pool<ConnectionManager<SqliteConnection>>> {
    let db_path = get_db_path(data_path, db_name);

    if let Some(parent_dir) = db_path.parent() {
        fs::create_dir_all(parent_dir)?;
    }
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(db_path.clone())?;

    let manager = ConnectionManager::<SqliteConnection>::new(
        db_path
            .to_str()
            .ok_or(anyhow::anyhow!("Invalid database path provided"))?,
    );

    let pool = Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool");

    Ok(pool)
}

pub fn open_main_db(data_path: &Path) -> anyhow::Result<MainDbInterface> {
    let pool = init_db_pool(data_path, "main")?;

    let mut conn = pool.get()?;

    conn.run_pending_migrations(MAIN_MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;

    let interface = MainDbInterface::new(pool);

    Ok(interface)
}
