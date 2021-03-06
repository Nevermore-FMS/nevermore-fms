use crate::{
    field::network::{
        AllianceStationToConfiguration, NetworkConfiguratorInfo, Reply,
        ThreadSafeNetworkConfigurator, ThreadSafeNetworkConfiguratorMap,
    },
    models::ThreadSafeDatabase,
};
use deno_core::{include_js_files, op_async, Extension, OpState, Resource, ResourceId};
use serde::Deserialize;
use std::vec;
use std::{cell::RefCell, rc::Rc};

pub fn init(
    database: ThreadSafeDatabase,
    network_configurator_map: ThreadSafeNetworkConfiguratorMap,
) -> Extension {
    Extension::builder()
        .js(include_js_files!(
            prefix "deno:extensions/nevermore-network",
            "runtime/js/04-network.js",
        ))
        .ops(vec![
            (
                "op_register_configurator",
                op_async(op_register_configurator),
            ),
            ("op_next_scan", op_async(op_next_scan)),
            ("op_reply_scan", op_async(op_reply_scan)),
            (
                "op_next_initial_configuration",
                op_async(op_next_initial_configuration),
            ),
            (
                "op_reply_initial_configuration",
                op_async(op_reply_initial_configuration),
            ),
            (
                "op_next_match_configuration",
                op_async(op_next_match_configuration),
            ),
            (
                "op_reply_match_configuration",
                op_async(op_reply_match_configuration),
            ),
        ])
        .state(move |state| {
            state.put(database.clone());
            state.put(network_configurator_map.clone());
            Ok(())
        })
        .build()
}

struct NetworkConfiguratoResource {
    configurator: ThreadSafeNetworkConfigurator,
}

impl Resource for NetworkConfiguratoResource {}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterArgs {
    info: NetworkConfiguratorInfo,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyArgs {
    id: ResourceId,
    reply: Option<String>,
}

pub async fn op_register_configurator(
    state: Rc<RefCell<OpState>>,
    args: RegisterArgs,
    _: (),
) -> anyhow::Result<ResourceId> {
    let mut borrowed_state = state.try_borrow_mut()?;
    let network_configurator_map = borrowed_state
        .try_borrow::<ThreadSafeNetworkConfiguratorMap>()
        .ok_or(anyhow::anyhow!("ThreadSafeNetworkConfiguratorMap has been borrowed"))?;

    let configurator = network_configurator_map.write().await.register(args.info);

    let id = borrowed_state
        .resource_table
        .add(NetworkConfiguratoResource { configurator });

    Ok(id)
}

pub async fn op_next_scan(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<()> {
    let borrowed_state = state.try_borrow()?;

    let configurator = borrowed_state
        .resource_table
        .get::<NetworkConfiguratoResource>(id)?;
    let mut rx = configurator.configurator.read().await.subscribe_scan();

    Ok(rx.recv().await?)
}

pub async fn op_reply_scan(
    state: Rc<RefCell<OpState>>,
    args: ReplyArgs,
    _: (),
) -> anyhow::Result<()> {
    let borrowed_state = state.try_borrow()?;

    let configurator = borrowed_state
        .resource_table
        .get::<NetworkConfiguratoResource>(args.id)?;
    configurator
        .configurator
        .read()
        .await
        .reply_scan(to_reply(args.reply));

    Ok(())
}

pub async fn op_next_initial_configuration(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<()> {
    let borrowed_state = state.try_borrow()?;

    let configurator = borrowed_state
        .resource_table
        .get::<NetworkConfiguratoResource>(id)?;
    let mut rx = configurator
        .configurator
        .read()
        .await
        .subscribe_initial_configuration();

    Ok(rx.recv().await?)
}

pub async fn op_reply_initial_configuration(
    state: Rc<RefCell<OpState>>,
    args: ReplyArgs,
    _: (),
) -> anyhow::Result<()> {
    let borrowed_state = state.try_borrow()?;

    let configurator = borrowed_state
        .resource_table
        .get::<NetworkConfiguratoResource>(args.id)?;
    configurator
        .configurator
        .read()
        .await
        .reply_initial_configuration(to_reply(args.reply));

    Ok(())
}

pub async fn op_next_match_configuration(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<AllianceStationToConfiguration> {
    let borrowed_state = state.try_borrow()?;

    let configurator = borrowed_state
        .resource_table
        .get::<NetworkConfiguratoResource>(id)?;
    let mut rx = configurator
        .configurator
        .read()
        .await
        .subscribe_match_configuration();
    let map = rx.recv().await?;

    Ok(map)
}

pub async fn op_reply_match_configuration(
    state: Rc<RefCell<OpState>>,
    args: ReplyArgs,
    _: (),
) -> anyhow::Result<()> {
    let borrowed_state = state.try_borrow()?;

    let configurator = borrowed_state
        .resource_table
        .get::<NetworkConfiguratoResource>(args.id)?;
    configurator
        .configurator
        .read()
        .await
        .reply_match_configuration(to_reply(args.reply));

    Ok(())
}

fn to_reply(old_reply: Option<String>) -> Reply {
    if old_reply.is_some() {
        Reply::ERROR(old_reply.unwrap())
    } else {
        Reply::SUCCESS
    }
}
