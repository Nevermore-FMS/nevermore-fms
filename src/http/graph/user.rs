use async_graphql::connection::*;
use async_graphql::*;
use async_graphql::guard::Guard;

use crate::application::ThreadSafeApplication;
use crate::models::user::User;
use crate::http::graph::guards::UserTypeGuard;
use crate::models::user::UserType;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
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
}