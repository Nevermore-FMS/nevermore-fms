use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
};

use diesel::{prelude::*, r2d2::{ConnectionManager, Pool}};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use directories::ProjectDirs;

use crate::database::main::interface::{MainDbInterface};

const MAIN_MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/main");

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

fn init_db_pool(
    override_default_path: Option<PathBuf>,
    db_name: &str,
) -> anyhow::Result<Pool<ConnectionManager<SqliteConnection>>> {
    let db_path = get_db_path(override_default_path, db_name)?;

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

pub fn open_main_db(override_default_path: Option<PathBuf>) -> anyhow::Result<MainDbInterface> {
    let pool = init_db_pool(override_default_path, "main")?;

    let mut conn = pool.get()?;

    conn.run_pending_migrations(MAIN_MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("{}", e.to_string()))?;

    let interface = MainDbInterface::new(pool);

    Ok(interface)
}
