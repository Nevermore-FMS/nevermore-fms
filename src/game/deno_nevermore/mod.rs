use crate::field::driverstation::ConfirmedState;
use crate::field::driverstation::State;
use crate::field::driverstation::ThreadSafeDriverStation;
use crate::field::enums::AllianceStation;
use crate::field::ThreadSafeField;
use crate::pub_sub::ThreadSafePubSub;
use async_graphql::*;
use deno_core::{include_js_files, op_async, op_sync, Extension, OpState, Resource, ResourceId};
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::HashMap;
use std::pin::Pin;
use std::rc::Rc;
use std::vec;
use tokio::sync::broadcast::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio_stream::{Stream, StreamExt};
use log::debug;

pub fn init(
    field: ThreadSafeField,
    pub_sub: ThreadSafePubSub,
    logger: Sender<LogMessage>,
) -> Extension {
    Extension::builder()
        .js(include_js_files!(
            prefix "deno:extensions/nevermore",
            "js/01-nevermore.js",
            "js/02-bootstrap.js",
        ))
        .ops(vec![
            ("op_log", op_sync(op_log)),
            ("op_publish", op_async(op_publish)),
            ("op_subscribe", op_async(op_subscribe)),
            ("op_unsubscribe", op_async(op_unsubscribe)),
            ("op_subscription_next", op_async(op_subscription_next)),
            ("op_tick_subscribe", op_async(op_tick_subscribe)),
            (
                "op_tick_subscription_next",
                op_async(op_tick_subscription_next),
            ),
            ("op_close_subscribe", op_async(op_close_subscribe)),
            (
                "op_close_subscription_next",
                op_async(op_close_subscription_next),
            ),
            ("op_get_driver_station", op_async(op_get_driver_station)),
            ("op_get_driver_station_team_numbers", op_async(op_get_driver_station_team_numbers)),
            ("op_add_team", op_async(op_add_team)),
            ("op_remove_team", op_async(op_remove_team)),
            (
                "op_set_emergency_stop_all",
                op_async(op_set_emergency_stop_all),
            ),
            ("op_set_enabled_all", op_async(op_set_enabled_all)),
            ("op_get_team", op_async(op_get_team)),
            ("op_get_team_map", op_async(op_get_team_map)),
            (
                "op_driverstation_get_confirmed_state",
                op_async(op_driverstation_get_confirmed_state),
            ),
            (
                "op_driverstation_get_state",
                op_async(op_driverstation_get_state),
            ),
            (
                "op_driverstation_set_state",
                op_async(op_driverstation_set_state),
            ),
            (
                "op_driverstation_is_in_correct_station",
                op_async(op_driverstation_is_in_correct_station),
            ),
            (
                "op_driverstation_is_in_match",
                op_async(op_driverstation_is_in_match),
            ),
            (
                "op_driverstation_get_address",
                op_async(op_driverstation_get_address),
            ),
            (
                "op_driverstation_has_closed",
                op_async(op_driverstation_has_closed),
            ),
        ])
        .state(move |state| {
            state.put(pub_sub.clone());
            state.put(field.clone());
            state.put(logger.clone());
            Ok(())
        })
        .build()
}

// Events -->

#[derive(Clone, Debug, Deserialize, SimpleObject)]
#[serde(rename_all = "camelCase")]
pub struct LogMessage {
    pub calling_function: String,
    pub file_name: String,
    pub message: String,
    pub level: u16,
    pub date_time: String,
}

pub fn op_log(state: &mut OpState, message: LogMessage, _: ()) -> anyhow::Result<()> {
    let mut logger = state.try_borrow::<Sender<LogMessage>>();
    debug!("{}: {}", message.message, message.level);

    if let Some(logger) = logger.take() {
        logger.send(message).ok();
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishArgs {
    topic: String,
    message: String,
}

pub async fn op_publish(
    state: Rc<RefCell<OpState>>,
    args: PublishArgs,
    _: (),
) -> anyhow::Result<()> {
    let borrowed_state = state.try_borrow()?;
    let pub_sub = borrowed_state
        .try_borrow::<ThreadSafePubSub>()
        .ok_or(anyhow::anyhow!("pub_sub has been dropped"))?;

    pub_sub.publish(args.topic, args.message).await?;

    Ok(())
}

struct StreamResource {
    topic: String,
    stream: Mutex<Pin<Box<dyn Stream<Item = String>>>>,
}

impl Resource for StreamResource {}

pub async fn op_subscribe(
    state: Rc<RefCell<OpState>>,
    topic: String,
    _: (),
) -> anyhow::Result<ResourceId> {
    let pub_sub = {
        let borrowed_field = state.try_borrow()?;
        borrowed_field
            .try_borrow::<ThreadSafePubSub>()
            .ok_or(anyhow::anyhow!("pub_sub has been dropped"))?
            .clone()
    };

    let id = state.try_borrow_mut()?.resource_table.add(StreamResource {
        topic: topic.clone(),
        stream: Mutex::new(pub_sub.subscribe(topic.clone()).await),
    });

    Ok(id)
}

pub async fn op_unsubscribe(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<ResourceId> {
    let pub_sub = {
        let borrowed_field = state.try_borrow()?;
        borrowed_field
            .try_borrow::<ThreadSafePubSub>()
            .ok_or(anyhow::anyhow!("pub_sub has been dropped"))?
            .clone()
    };

    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<StreamResource>(id)
            .ok_or(anyhow::anyhow!("subscription already dropped"))?
    };

    state.try_borrow_mut()?.resource_table.close(id);

    pub_sub.unsubscribe(resource.topic.clone()).await;

    Ok(id)
}

pub async fn op_subscription_next(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<String> {
    let resource = state
        .try_borrow()?
        .resource_table
        .get::<StreamResource>(id)
        .ok_or(anyhow::anyhow!("subscription already dropped"))?;

    let mut stream = resource.stream.lock().await;

    stream
        .next()
        .await
        .ok_or(anyhow::anyhow!("non-existent message"))
}

struct ReceiverResource {
    receiver: Mutex<Receiver<()>>,
}

impl Resource for ReceiverResource {}

pub async fn op_tick_subscribe(
    state: Rc<RefCell<OpState>>,
    _: (),
    _: (),
) -> anyhow::Result<ResourceId> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    let id = state
        .try_borrow_mut()?
        .resource_table
        .add(ReceiverResource {
            receiver: Mutex::new(field.lock().await.subscribe_to_tick_channel()?),
        });

    Ok(id)
}

pub async fn op_tick_subscription_next(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<()> {
    let ticker = state
        .try_borrow()?
        .resource_table
        .get::<ReceiverResource>(id)
        .ok_or(anyhow::anyhow!("non-existent subscription"))?;

    ticker.receiver.lock().await.recv().await?;

    Ok(())
}

pub async fn op_close_subscribe(
    state: Rc<RefCell<OpState>>,
    _: (),
    _: (),
) -> anyhow::Result<ResourceId> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    let id = state
        .try_borrow_mut()?
        .resource_table
        .add(ReceiverResource {
            receiver: Mutex::new(field.lock().await.subscribe_to_close_channel()?),
        });

    Ok(id)
}

pub async fn op_close_subscription_next(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<()> {
    let closer = state
        .try_borrow()?
        .resource_table
        .get::<ReceiverResource>(id)
        .ok_or(anyhow::anyhow!("non-existent subscription"))?;

    closer.receiver.lock().await.recv().await?;

    Ok(())
}

// Field -->
struct DriverStationResource {
    driver_station: ThreadSafeDriverStation,
}

impl Resource for DriverStationResource {}

pub async fn op_get_driver_station(
    state: Rc<RefCell<OpState>>,
    team_number: u16,
    _: (),
) -> anyhow::Result<ResourceId> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    let id = state
        .try_borrow_mut()?
        .resource_table
        .add(DriverStationResource {
            driver_station: field.lock().await.get_driver_station(team_number).await?,
        });

    Ok(id)
}

pub async fn op_get_driver_station_team_numbers(
    state: Rc<RefCell<OpState>>,
    _: (),
    _: (),
) -> anyhow::Result<Vec<u16>> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    let ids = field.lock().await.driver_station_team_numbers().await?;

    Ok(ids)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTeamArgs {
    team_number: u16,
    alliance_station: i32,
}

pub async fn op_add_team(
    state: Rc<RefCell<OpState>>,
    args: AddTeamArgs,
    _: (),
) -> anyhow::Result<()> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    field
        .lock()
        .await
        .add_team(
            args.team_number,
            AllianceStation::from_integer(args.alliance_station),
        )
        .await?;

    Ok(())
}

pub async fn op_remove_team(
    state: Rc<RefCell<OpState>>,
    team_number: u16,
    _: (),
) -> anyhow::Result<()> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    field.lock().await.remove_team(team_number).await?;

    Ok(())
}

pub async fn op_set_emergency_stop_all(
    state: Rc<RefCell<OpState>>,
    emergency_stopped: bool,
    _: (),
) -> anyhow::Result<()> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    field
        .lock()
        .await
        .set_emergency_stop_all(emergency_stopped)
        .await?;

    Ok(())
}

pub async fn op_set_enabled_all(
    state: Rc<RefCell<OpState>>,
    enabled: bool,
    _: (),
) -> anyhow::Result<()> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    field.lock().await.set_enabled_all(enabled).await?;

    Ok(())
}

pub async fn op_get_team(
    state: Rc<RefCell<OpState>>,
    team_number: u16,
    _: (),
) -> anyhow::Result<i32> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    let alliance_station = field
        .lock()
        .await
        .get_team_alliance_station(team_number)
        .await?
        .to_integer();

    Ok(alliance_station)
}

pub async fn op_get_team_map(
    state: Rc<RefCell<OpState>>,
    _: (),
    _: (),
) -> anyhow::Result<HashMap<u16, i32>> {
    let field = {
        let borrowed_state = state.try_borrow()?;
        borrowed_state
            .try_borrow::<ThreadSafeField>()
            .ok_or(anyhow::anyhow!("field has been dropped"))?
            .clone()
    };

    let alliance_station_old_map = field.lock().await.get_team_alliance_station_map().await?;

    let mut alliance_station_map: HashMap<u16, i32> = HashMap::new();

    for (key, value) in alliance_station_old_map.iter() {
        alliance_station_map.insert(*key, value.to_integer());
    }

    Ok(alliance_station_map)
}

pub async fn op_driverstation_get_confirmed_state(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<ConfirmedState> {
    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<DriverStationResource>(id)
            .ok_or(anyhow::anyhow!("driverstation already dropped"))?
    };

    let confirmed_state = resource.driver_station.lock().await.get_confirmed_state()?;

    Ok(confirmed_state)
}

pub async fn op_driverstation_get_state(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<State> {
    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<DriverStationResource>(id)
            .ok_or(anyhow::anyhow!("driverstation already dropped"))?
    };

    let ds_state = resource.driver_station.lock().await.get_state().await?;

    Ok(ds_state)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DriverStationSetStateArgs {
    rid: ResourceId,
    state: State,
}

pub async fn op_driverstation_set_state(
    state: Rc<RefCell<OpState>>,
    args: DriverStationSetStateArgs,
    _: (),
) -> anyhow::Result<()> {
    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<DriverStationResource>(args.rid)
            .ok_or(anyhow::anyhow!("driverstation already dropped"))?
    };

    resource
        .driver_station
        .lock()
        .await
        .set_state(args.state)
        .await;

    Ok(())
}

pub async fn op_driverstation_is_in_correct_station(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<bool> {
    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<DriverStationResource>(id)
            .ok_or(anyhow::anyhow!("driverstation already dropped"))?
    };

    let is_in_correct_station = resource
        .driver_station
        .lock()
        .await
        .is_in_correct_station()
        .await?;

    Ok(is_in_correct_station)
}

pub async fn op_driverstation_is_in_match(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<bool> {
    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<DriverStationResource>(id)
            .ok_or(anyhow::anyhow!("driverstation already dropped"))?
    };

    let is_in_match = resource.driver_station.lock().await.is_in_match().await?;

    Ok(is_in_match)
}

pub async fn op_driverstation_get_address(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<String> {
    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<DriverStationResource>(id)
            .ok_or(anyhow::anyhow!("driverstation already dropped"))?
    };

    let is_in_match = resource.driver_station.lock().await.address();

    Ok(is_in_match.to_string())
}

pub async fn op_driverstation_has_closed(
    state: Rc<RefCell<OpState>>,
    id: ResourceId,
    _: (),
) -> anyhow::Result<bool> {
    let resource = {
        state
            .try_borrow()?
            .resource_table
            .get::<DriverStationResource>(id)
            .ok_or(anyhow::anyhow!("driverstation already dropped"))?
    };

    let has_closed = resource.driver_station.lock().await.has_closed();

    Ok(has_closed)
}
