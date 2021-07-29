use futures::channel::mpsc::UnboundedSender;
use log::info;
use std::{sync::Arc, thread::JoinHandle};

use crate::{
    field::{Field, ThreadSafeField},
    models::{
        config::{Config, ConfigKey},
        plugin::PluginType,
    },
    plugin::{deno_nevermore::LogMessage, DenoPluginRuntime},
    pub_sub::{PubSub, ThreadSafePubSub},
    session::{SessionStorage, ThreadSafeSessionStorage},
};

use crate::models::plugin::Plugin;
use crate::models::{Database, ThreadSafeDatabase};
use chrono::Local;
use deno_core::InspectorSessionProxy;
use tokio::sync::{
    broadcast::{channel, Receiver, Sender},
    RwLock,
};

pub type ThreadSafeApplication = Arc<RwLock<Application>>;

pub struct Application {
    pub field: ThreadSafeField,
    pub deno_pub_sub: ThreadSafePubSub,
    pub database: ThreadSafeDatabase,
    deno_handle: Option<JoinHandle<()>>,
    pub closing_sender: Option<Sender<()>>,
    pub log_sender: tokio::sync::broadcast::Sender<LogMessage>,
    pub inspector_sender: Option<UnboundedSender<InspectorSessionProxy>>,
    pub session_storage: ThreadSafeSessionStorage,
}

impl Application {
    pub async fn new(db_uri: Option<String>) -> anyhow::Result<ThreadSafeApplication> {
        let deno_pub_sub = PubSub::new();
        let database = Database::new(true, false, db_uri).await?;
        let field = Field::new(
            Config::get(database.clone(), ConfigKey::EventName)
                .await
                .unwrap_or("nevermore".to_string()),
        )
        .await?;
        let (log_sender, _) = tokio::sync::broadcast::channel::<LogMessage>(10);

        let application = Self {
            field,
            deno_pub_sub,
            database: database.clone(),
            deno_handle: None,
            closing_sender: None,
            log_sender: log_sender.clone(),
            inspector_sender: None,
            session_storage: SessionStorage::new(),
        };

        let application = Arc::new(RwLock::new(application));

        application
            .write()
            .await
            .restart_deno_runtime(application.clone());

        Ok(application)
    }

    pub fn subscribe_to_log(&mut self) -> tokio::sync::broadcast::Receiver<LogMessage> {
        self.log_sender.subscribe()
    }

    pub fn restart_deno_runtime(&mut self, application: ThreadSafeApplication) {
        if let Some(sender) = self.closing_sender.take() {
            if let Some(join_handle) = self.deno_handle.take() {
                sender.send(()).ok();
                join_handle.join().ok();
            }
        }

        let (tx, rx) = channel(2);

        let join_handle = std::thread::spawn(move || {
            let rt = create_basic_runtime();
            let local = tokio::task::LocalSet::new();
            local.block_on(&rt, run_deno(rx, application))
        });

        self.closing_sender = Some(tx);
        self.deno_handle = Some(join_handle);
    }
}

async fn run_deno(mut closing_receiver: Receiver<()>, application: ThreadSafeApplication) {
    tokio::select! {
        _ = run_event_loop_forever(application) => {}
        _ = closing_receiver.recv() => {}
    }
}

async fn run_event_loop_forever(application: ThreadSafeApplication) {
    loop {
        let (log_sender, database, deno_runtime_safe) = {
            let mut locked_application = application.write().await;
            locked_application
                .field
                .read()
                .await
                .network_configurator_map()
                .write()
                .await
                .clear();
            let configurator_map = locked_application.field.clone().read().await.network_configurator_map();
            let deno_runtime_safe = DenoPluginRuntime::new(
                locked_application.field.clone(),
                locked_application.database.clone(),
                configurator_map,
                locked_application.deno_pub_sub.clone(),
                locked_application.log_sender.clone(),
            )
            .expect("deno runtime couldn't be made");
            locked_application.inspector_sender =
                Some(deno_runtime_safe.read().await.get_session_sender());
            (
                locked_application.log_sender.clone(),
                locked_application.database.clone(),
                deno_runtime_safe,
            )
        };
        let mut deno_runtime = deno_runtime_safe.write().await;
        let mut plugins = Plugin::get_all_to_load(database.clone()).await.ok();
        let mut has_game = false;
        if let Some(plugins) = plugins.take() {
            for plugin in plugins {
                if plugin.plugin_type == PluginType::Game && has_game {
                    send_log_error_message(
                        log_sender.clone(),
                        format!(
                            "Not loading plugin '{}'. A game plugin has already been loaded!",
                            plugin.name
                        )
                        .to_string(),
                    );
                    continue;
                }
                if plugin.plugin_type == PluginType::Game {
                    has_game = true;
                }
                let result = deno_runtime.run_code(plugin.name.clone(), plugin.code);
                if result.is_err() {
                    send_log_error_message(
                        log_sender.clone(),
                        format!("Compilation Error: {}", result.err().unwrap()).to_string(),
                    );
                }
            }
        }

        let result = deno_runtime.run_event_loop().await;
        if result.is_err() {
            send_log_error_message(
                log_sender.clone(),
                format!("Runtime Error: {}", result.err().unwrap()).to_string(),
            );
        }

        send_log_error_message(
            log_sender.clone(),
            "Plugin runtime exited early, restarting in 15 seconds...".to_string(),
        );
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        send_log_error_message(log_sender.clone(), "Restarting worker...\n\n".to_string());
    }
}

fn send_log_error_message(log_sender: tokio::sync::broadcast::Sender<LogMessage>, message: String) {
    info!("[Plugin Runtime] {}", message);
    log_sender
        .send(LogMessage {
            message: message,
            level: 3,
            date_time: Local::now()
                .format("%-m/%-d/%-Y, %-I:%-M:%S %p")
                .to_string(),
        })
        .ok();
}

pub fn create_basic_runtime() -> tokio::runtime::Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        // This limits the number of threads for blocking operations (like for
        // synchronous fs ops) or CPU bound tasks like when we run dprint in
        // parallel for deno fmt.
        // The default value is 512, which is an unhelpfully large thread pool. We
        // don't ever want to have more than a couple dozen threads.
        .max_blocking_threads(32)
        .build()
        .unwrap()
}
