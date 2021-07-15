use async_graphql::connection::*;
use async_graphql::*;

#[cfg(feature = "developer")]
use crate::models::worker::CreateWorkerParams;
use crate::{application::ThreadSafeApplication, models::{user::User, worker::Worker}};

pub type NevermoreSchema = Schema<Query, Mutation, Subscription>;

pub struct Query;

#[Object]
impl Query {
    async fn node<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        id: ID
    ) -> Result<Node> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let db = app.lock().await.database.clone();
        let (type_name, id) = decode_id(id)?;
        match type_name.as_str() {
            "Worker" => {
                Ok(Node::Worker(Worker::get(db, id).await?))
            }
            "User" => {
                Ok(Node::User(User::get(db, id).await?))
            }
            _ => {
                Err(Error::new("unknown type_name"))
            }
        }
    }

    async fn users<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, User, EmptyFields, EmptyFields>> {
        query(
            after,
            before,
            first,
            last,
            |after, before, mut first, mut last| async move {
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
                let (has_prev_page, has_next_page, users) =
                    User::get_all_paginated(db, is_inverted, number_of_docs, after, before)
                        .await?;
                let mut connection: Connection<String, User> =
                    Connection::new(has_prev_page, has_next_page);
                connection.append(
                    users
                        .into_iter()
                        .map(|user| Edge::new(user.username.clone(), user)),
                );

                Ok(connection)
            },
        )
        .await
    }

    async fn workers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Worker, EmptyFields, EmptyFields>> {
        query(
            after,
            before,
            first,
            last,
            |after, before, mut first, mut last| async move {
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
                let (has_prev_page, has_next_page, workers) =
                    Worker::get_all_paginated(db, is_inverted, number_of_docs, after, before)
                        .await?;
                let mut connection: Connection<String, Worker> =
                    Connection::new(has_prev_page, has_next_page);
                connection.append(
                    workers
                        .into_iter()
                        .map(|worker| Edge::new(worker.name.clone(), worker)),
                );

                Ok(connection)
            },
        )
        .await
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
        app_locked.restart_deno_worker(app.clone());
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

#[derive(Interface)]
#[graphql(
    field(name = "id", type = "ID"),
)]
enum Node {
    Worker(Worker),
    User(User),
}

pub fn decode_id(id: ID) -> anyhow::Result<(String, String)> {
    let b64_id = id.to_string();
    let id_bytes = base64::decode(b64_id)?;
    let id_string = String::from_utf8(id_bytes)?;
    let parts: Vec<&str> = id_string.split("|").collect();
    if parts.len() != 2 {
        return Err(anyhow::anyhow!("incorrectly formatted id."))
    };
    Ok((parts[0].to_string(), parts[1].to_string()))
}

pub fn create_schema(application: ThreadSafeApplication) -> NevermoreSchema {
    Schema::build(Query, Mutation, Subscription)
        .data(application)
        .finish()
}
