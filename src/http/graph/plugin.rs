use async_graphql::connection::*;
use async_graphql::*;
use async_graphql::guard::Guard;

use crate::application::ThreadSafeApplication;
use crate::models::plugin::Plugin;
use crate::http::graph::guards::UserTypeGuard;
use crate::models::user::UserType;

#[derive(Default)]
pub struct PluginQuery;

#[Object]
impl PluginQuery {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn plugins<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<String, Plugin, EmptyFields, EmptyFields>> {
        query(
            after,
            before,
            first,
            last,
            |after, before, mut first, mut last| async move {
                let app = ctx.data::<ThreadSafeApplication>()?;
                let db = app.read().await.database.clone();
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
                    Plugin::get_all_paginated(db, is_inverted, number_of_docs, after, before)
                        .await?;
                let mut connection: Connection<String, Plugin> =
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
}

