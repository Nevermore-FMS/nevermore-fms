use async_graphql::*;
use tokio_stream::StreamExt;

use crate::application::ThreadSafeApplication;
use crate::database::worker::{CreateWorkerParams, Worker};
use crate::game::deno_nevermore::LogMessage;

pub type NevermoreSchema = Schema<Query, Mutation, Subscription>;

pub struct Query;

#[Object]
impl Query {
    async fn workers<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Worker>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.lock().await;
        Ok(Worker::get_all_workers(app_locked.database.clone()).await?)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    // Development Mutations without Auth :)

    #[cfg(debug_assertions)]
    async fn restart_worker<'ctx>(&self, ctx: &Context<'ctx>) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let mut app_locked = app.lock().await;
        let log_sender = app_locked.log_sender.clone();
        let database = app_locked.database.clone();
        app_locked.restart_deno_worker(true, database, log_sender);
        Ok(true)
    }

    #[cfg(debug_assertions)]
    async fn create_worker<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: CreateWorkerParams,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.lock().await;
        Worker::create(app_locked.database.clone(), params).await?;
        Ok(true)
    }

    #[cfg(debug_assertions)]
    async fn delete_worker<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.lock().await;
        Worker::delete(app_locked.database.clone(), name).await?;
        Ok(true)
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    async fn log<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<impl tokio_stream::Stream<Item = Result<LogMessage>>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let mut app_locked = app.lock().await;
        Ok(
            tokio_stream::wrappers::BroadcastStream::new(app_locked.subscribe_to_log())
                .map(|x| Ok(x?)),
        )
    }
}

pub fn create_schema(application: ThreadSafeApplication) -> NevermoreSchema {
    Schema::build(Query, Mutation, Subscription)
        .data(application)
        .finish()
}
