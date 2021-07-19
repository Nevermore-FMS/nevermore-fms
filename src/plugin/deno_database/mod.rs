use crate::models::{Database, ThreadSafeDatabase};
use deno_core::{include_js_files, op_async, Extension, OpState, Resource, ResourceId};
use serde::Deserialize;
use serde_json::Value;
use std::rc::Rc;
use std::vec;
use std::{cell::RefCell, collections::HashMap};

pub fn init() -> Extension {
    Extension::builder()
        .js(include_js_files!(
            prefix "deno:extensions/nevermore-database",
            "runtime/js/03-database.js",
        ))
        .ops(vec![
            ("op_create_database", op_async(op_create_database)),
            ("op_database_run", op_async(op_database_run)),
            ("op_database_get", op_async(op_database_get)),
            ("op_database_all", op_async(op_database_all)),
        ])
        .build()
}

struct DatabaseResource {
    database: ThreadSafeDatabase,
}

impl Resource for DatabaseResource {}

pub async fn op_create_database(
    state: Rc<RefCell<OpState>>,
    name: String,
    _: (),
) -> anyhow::Result<ResourceId> {
    let mut borrowed_state = state.try_borrow_mut()?;

    let database = Database::new(false, false, Some(format!("db/{}.db", name))).await?;

    let rid = borrowed_state
        .resource_table
        .add(DatabaseResource { database });

    Ok(rid)
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryArgs {
    rid: ResourceId,
    stmt: String,
    params: Vec<Value>,
}

pub async fn op_database_run(
    state: Rc<RefCell<OpState>>,
    args: QueryArgs,
    _: (),
) -> anyhow::Result<()> {
    let borrowed_state = state.try_borrow()?;
    let database_resource = borrowed_state
        .resource_table
        .get::<DatabaseResource>(args.rid)
        .ok_or(anyhow::anyhow!("database not found"))?;

    let db = database_resource.database.clone();

    db.lock().await.execute(args.stmt, args.params).await?;

    Ok(())
}

pub async fn op_database_get(
    state: Rc<RefCell<OpState>>,
    args: QueryArgs,
    _: (),
) -> anyhow::Result<HashMap<String, Value>> {
    let borrowed_state = state.try_borrow()?;
    let database_resource = borrowed_state
        .resource_table
        .get::<DatabaseResource>(args.rid)
        .ok_or(anyhow::anyhow!("database not found"))?;

    let db = database_resource.database.clone();

    let value = db.lock().await.query_row(args.stmt, args.params).await?;

    Ok(value)
}

pub async fn op_database_all(
    state: Rc<RefCell<OpState>>,
    args: QueryArgs,
    _: (),
) -> anyhow::Result<Vec<HashMap<String, Value>>> {
    let borrowed_state = state.try_borrow()?;
    let database_resource = borrowed_state
        .resource_table
        .get::<DatabaseResource>(args.rid)
        .ok_or(anyhow::anyhow!("database not found"))?;

    let db = database_resource.database.clone();

    let values = db.lock().await.query_rows(args.stmt, args.params).await?;

    Ok(values)
}
