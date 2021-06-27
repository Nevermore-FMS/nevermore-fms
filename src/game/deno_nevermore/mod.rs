use crate::nevermore::Nevermore;
use crate::robot::ThreadSafeRobot;
use deno_core::include_js_files;
use deno_core::{op_async, op_sync, Extension, OpState, Resource, ResourceId};
use serde::Deserialize;
use std::borrow::Cow;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::Mutex;

pub fn init() -> Extension {
    Extension::builder()
        .js(include_js_files!(
            prefix "deno:extensions/nevermore",
            "js/01-nevermore.js",
            "js/02-main.js",
        ))
        .ops(vec![("op_log", op_sync(op_log))])
        .state(move |state| {
            //state.put(nevermore);
            Ok(())
        })
        .build()
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogArgs {
    msg: String,
    level: u16,
}

pub fn op_log(state: &mut OpState, args: LogArgs, _: ()) -> anyhow::Result<()> {
    if args.level > 1 {
        error!("{}", args.msg);
    } else {
        info!("{}", args.msg);
    }

    Ok(())
}

/*pub async fn op_next_tick(state: Rc<RefCell<OpState>>, args: (), _: ()) -> anyhow::Result<()> {
    let mut mutable_state = state.borrow_mut();
    let nevermore = mutable_state.try_borrow_mut::<Arc<Mutex<Nevermore>>>().ok_or(anyhow::anyhow!("nevermore has been dropped"))?;

    nevermore.lock().await.ticker_sender.clone().subscribe().recv().await?;

    Ok(())
}

pub async fn op_next_close(state: Rc<RefCell<OpState>>, args: (), _: ()) -> anyhow::Result<()> {
    let mut mutable_state = state.borrow_mut();
    let nevermore = mutable_state.try_borrow_mut::<Arc<Mutex<Nevermore>>>().ok_or(anyhow::anyhow!("nevermore has been dropped"))?;

    nevermore.lock().await.closing_sender.clone().unwrap().subscribe().recv().await?;

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTeamResource {
    team_number: u16,
}

struct ThreadSafeRobotResource {
    thread_safe_robot: ThreadSafeRobot
}

impl Resource for ThreadSafeRobotResource {
    fn name(&self) -> Cow<str> {
        "threadSafeRobot".into()
    }
}

pub async fn op_get_team_resource(state: Rc<RefCell<OpState>>, args: GetTeamResource, _: ()) -> anyhow::Result<ResourceId> {
    let mut mutable_state = state.borrow_mut();
    let nevermore = mutable_state.try_borrow_mut::<Arc<Mutex<Nevermore>>>().ok_or(anyhow::anyhow!("nevermore has been dropped"))?;

    let team_number_to_robot = {
        let locked_nevermore = nevermore.lock().await;
        locked_nevermore.team_number_to_robot.clone().lock().await
    };

    let robot = team_number_to_robot.get(&args.team_number).ok_or(anyhow::anyhow!("couldn't find robot"))?;

    let id = state.borrow_mut().resource_table.add::<ThreadSafeRobotResource>(ThreadSafeRobotResource { thread_safe_robot: robot.clone() });

    Ok(id)
}*/
