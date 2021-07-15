use crate::pub_sub::ThreadSafePubSub;
use deno_core::{include_js_files, op_async, Extension, OpState, Resource, ResourceId};
use serde::Deserialize;
use std::cell::RefCell;
use std::pin::Pin;
use std::rc::Rc;
use std::vec;
use tokio::sync::Mutex;
use tokio_stream::{Stream, StreamExt};

pub fn init(pub_sub: ThreadSafePubSub) -> Extension {
    Extension::builder()
        .js(include_js_files!(
            prefix "deno:extensions/nevermore-pubsub",
            "runtime/js/02-pubsub.js",
        ))
        .ops(vec![
            ("op_publish", op_async(op_publish)),
            ("op_subscribe", op_async(op_subscribe)),
            ("op_unsubscribe", op_async(op_unsubscribe)),
            ("op_subscription_next", op_async(op_subscription_next)),
        ])
        .state(move |state| {
            state.put(pub_sub.clone());
            Ok(())
        })
        .build()
}

// Events -->

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
