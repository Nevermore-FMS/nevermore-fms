use deno_core::futures::StreamExt;
use flo_stream::{MessagePublisher, Publisher};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;

use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio_stream::Stream;

type PubSubMap = Arc<RwLock<HashMap<String, Publisher<Vec<u8>>>>>;

pub type ThreadSafePubSub = Arc<PubSub>;

pub struct PubSub {
    map: PubSubMap,
}

impl PubSub {
    pub fn new() -> ThreadSafePubSub {
        Arc::new(PubSub {
            map: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn subscribe<T>(&self, topic: String) -> Pin<Box<dyn Stream<Item = T> + Send>>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut map = self.map.write().await;
        let publisher = map.entry(topic.clone()).or_insert(Publisher::new(10));
        publisher
            .subscribe()
            .map(|x| bincode::deserialize::<T>(&x[..]).unwrap())
            .boxed()
    }

    pub async fn unsubscribe(&self, topic: String) {
        let mut map = self.map.write().await;
        let publisher = map.get(&topic);
        if publisher.is_some() {
            let publisher = publisher.unwrap();
            if publisher.count_subscribers() <= 0 {
                map.remove(&topic);
            }
        }
    }

    pub async fn publish<T>(&self, topic: String, message: T) -> anyhow::Result<()>
    where
        T: Serialize + DeserializeOwned,
    {
        let mut map = self.map.write().await;
        let publisher = map.get_mut(&topic);
        if publisher.is_some() {
            let publisher = publisher.unwrap();
            let binary = bincode::serialize(&message)?;
            publisher.publish(binary).await;
        }
        Ok(())
    }
}
