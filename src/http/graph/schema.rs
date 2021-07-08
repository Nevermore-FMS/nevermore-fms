use async_graphql::*;

use crate::application::ThreadSafeApplication;
#[cfg(feature = "developer")]
use crate::database::worker::{CreateWorkerParams, Worker};

pub type NevermoreSchema = Schema<Query, Mutation, Subscription>;

pub struct Query;

#[Object]
impl Query {
    #[cfg(feature = "developer")]
    async fn dev_workers<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Worker>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.lock().await;
        Ok(Worker::get_all_workers(app_locked.database.clone()).await?)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    // Development Mutations without Auth :)

    #[cfg(feature = "developer")]
    async fn dev_restart_worker<'ctx>(&self, ctx: &Context<'ctx>) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let mut app_locked = app.lock().await;
        app_locked.restart_deno_worker(true, app.clone());
        Ok(true)
    }

    #[cfg(feature = "developer")]
    async fn dev_create_worker<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: CreateWorkerParams,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.lock().await;
        Worker::create(app_locked.database.clone(), params).await?;
        Ok(true)
    }

    #[cfg(feature = "developer")]
    async fn dev_delete_worker<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.lock().await;
        Worker::delete(app_locked.database.clone(), name).await?;
        Ok(true)
    }
}

pub struct Subscription;

#[Subscription]
impl Subscription {
    #[cfg(feature = "developer")]
    async fn dev_log<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<impl tokio_stream::Stream<Item = Result<crate::game::deno_nevermore::LogMessage>>>
    {
        use tokio_stream::StreamExt;

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
