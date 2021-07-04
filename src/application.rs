use std::{sync::Arc, thread::JoinHandle};

use crate::{
    field::{Field, ThreadSafeField},
    game::{deno_nevermore::LogMessage, DenoWorker},
    pub_sub::{PubSub, ThreadSafePubSub},
};

use crate::database::worker::Worker;
use crate::database::{Database, ThreadSafeDatabase};
use deno_core::futures::channel::oneshot::{channel, Receiver, Sender};
use tokio::sync::Mutex;

pub type ThreadSafeApplication = Arc<Mutex<Application>>;

pub struct Application {
    pub field: ThreadSafeField,
    pub deno_pub_sub: ThreadSafePubSub,
    pub database: ThreadSafeDatabase,
    deno_handle: Option<JoinHandle<()>>,
    closing_sender: Option<Sender<()>>,
    pub log_sender: tokio::sync::broadcast::Sender<LogMessage>,
}

impl Application {
    pub async fn new() -> anyhow::Result<ThreadSafeApplication> {
        let field = Field::new("nevermore".to_string()).await?;

        let deno_pub_sub = PubSub::new();

        let database = Database::new(false, Some("test.db".to_string()))?;

        {
            database.clone().lock().await.create_tables();
        }

        let (log_sender, _) = tokio::sync::broadcast::channel::<LogMessage>(10);

        let mut application = Self {
            field,
            deno_pub_sub,
            database: database.clone(),
            deno_handle: None,
            closing_sender: None,
            log_sender: log_sender.clone(),
        };

        application.restart_deno_worker(true, database.clone(), log_sender);

        Ok(Arc::new(Mutex::new(application)))
    }

    pub fn subscribe_to_log(&mut self) -> tokio::sync::broadcast::Receiver<LogMessage> {
        self.log_sender.subscribe()
    }

    pub fn restart_deno_worker(
        &mut self,
        attach_inspector: bool,
        database: ThreadSafeDatabase,
        log_sender: tokio::sync::broadcast::Sender<LogMessage>,
    ) {
        if let Some(sender) = self.closing_sender.take() {
            if let Some(join_handle) = self.deno_handle.take() {
                sender.send(()).ok();
                join_handle.join().ok();
            }
        }

        let (tx, rx) = channel();

        let field = self.field.clone();
        let deno_pub_sub = self.deno_pub_sub.clone();

        let join_handle = std::thread::spawn(move || {
            let rt = create_basic_runtime();
            let local = tokio::task::LocalSet::new();
            local.block_on(
                &rt,
                run_deno(
                    rx,
                    field,
                    deno_pub_sub,
                    attach_inspector,
                    database,
                    log_sender,
                ),
            )
        });

        self.closing_sender = Some(tx);
        self.deno_handle = Some(join_handle);
    }
}

async fn run_deno(
    closing_receiver: Receiver<()>,
    field: ThreadSafeField,
    deno_pub_sub: ThreadSafePubSub,
    attach_inspector: bool,
    database: ThreadSafeDatabase,
    log_sender: tokio::sync::broadcast::Sender<LogMessage>,
) {
    let deno_worker = DenoWorker::new(
        field.clone(),
        deno_pub_sub.clone(),
        attach_inspector,
        log_sender,
    );
    let mut deno_worker = deno_worker.lock().await;
    let mut workers = Worker::get_all_workers_to_load(database).await.ok();
    if let Some(workers) = workers.take() {
        for worker in workers {
            deno_worker.run_code(worker.name, worker.code).ok();
        }
    }

    tokio::select! {
        _ = deno_worker.run_event_loop() => {}
        _ = closing_receiver => {}
    }
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
