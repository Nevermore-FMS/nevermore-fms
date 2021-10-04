use async_graphql::*;
use async_graphql::guard::Guard;
use tokio_stream::StreamExt;

use crate::application::ThreadSafeApplication;
use crate::models::user::UserType;

use super::guards::UserTypeGuard;

#[derive(Default)]
pub struct PubSubMutation {}

#[Object]
impl PubSubMutation {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Viewer")))]
    async fn publish<'ctx>(&self, ctx: &Context<'ctx>, topic: String, message: String) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;

        let pub_sub = app.read().await.deno_pub_sub.clone();
        pub_sub.publish(topic, message).await?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct PubSubSubscription {}

#[Subscription]
impl PubSubSubscription {
    async fn subscribe<'ctx>(&self, ctx: &Context<'ctx>, topic: String) -> Result<impl tokio_stream::Stream<Item = Result<String>>> {
        let stream = {
            let app = ctx.data::<ThreadSafeApplication>()?;

            let pub_sub = app.read().await.deno_pub_sub.clone();
            pub_sub.subscribe::<String>(topic).await.map(|x| Ok(x))
        };
        Ok(stream)
    }
}
