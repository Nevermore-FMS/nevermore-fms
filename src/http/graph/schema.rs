use async_graphql::*;
use async_graphql::connection::*;

use crate::{application::ThreadSafeApplication, database::worker::Worker};
#[cfg(feature = "developer")]
use crate::database::worker::CreateWorkerParams;

pub type NevermoreSchema = Schema<Query, Mutation, Subscription>;

pub struct Query;

#[Object]
impl Query {
    async fn workers<'ctx>(&self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Worker, EmptyFields, EmptyFields>> {
        query(after, before, first, last, |after, before, mut first, mut last| async move {
            let app = ctx.data::<ThreadSafeApplication>()?;
            let db = app.lock().await.database.clone();
            let mut is_inverted = false;
            let mut number_of_docs: usize = 10;
            if let Some(first) = first.take() {
                number_of_docs = first;
            }
            if let Some(last) = last.take() {
                is_inverted = true;
                number_of_docs = last;
            }
            let (has_prev_page, has_next_page, workers) = Worker::get_all_paginated(db, is_inverted, number_of_docs, after, before).await?;
            let mut connection: Connection<String, Worker> = Connection::new(has_prev_page, has_next_page);
            connection.append(workers.into_iter().map(|worker| {
                Edge::new(worker.name.clone(), worker)
            }));

            Ok(connection)
        }).await
    }

    #[cfg(feature = "developer")]
    async fn dev_workers<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Worker>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.lock().await;
        Ok(Worker::get_all(app_locked.database.clone()).await?)
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
    ) -> Result<impl tokio_stream::Stream<Item = Result<crate::worker::deno_nevermore::LogMessage>>>
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
