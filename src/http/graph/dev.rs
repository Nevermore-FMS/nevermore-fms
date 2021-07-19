use async_graphql::*;

#[cfg(feature = "developer")]
use crate::{
    application::ThreadSafeApplication, models::plugin::CreatePluginParams, models::plugin::Plugin,
};

#[derive(Default)]
pub struct DevQuery;

#[Object]
impl DevQuery {
    #[cfg(feature = "developer")]
    async fn dev_plugins<'ctx>(&self, ctx: &Context<'ctx>) -> Result<Vec<Plugin>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        Ok(Plugin::get_all(app_locked.database.clone()).await?)
    }
}

#[derive(Default)]
pub struct DevMutation;

#[Object]
impl DevMutation {
    // Development Mutations without Auth :)

    #[cfg(feature = "developer")]
    async fn dev_restart_plugin<'ctx>(&self, ctx: &Context<'ctx>) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let mut app_locked = app.write().await;
        app_locked.restart_deno_runtime(app.clone());
        Ok(true)
    }

    #[cfg(feature = "developer")]
    async fn dev_create_plugin<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        params: CreatePluginParams,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        Plugin::create(app_locked.database.clone(), params).await?;
        Ok(true)
    }

    #[cfg(feature = "developer")]
    async fn dev_delete_plugin<'ctx>(&self, ctx: &Context<'ctx>, name: String) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        Plugin::delete(app_locked.database.clone(), name).await?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct DevSubscription;

#[Subscription]
impl DevSubscription {
    #[cfg(feature = "developer")]
    async fn dev_log<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<impl tokio_stream::Stream<Item = Result<crate::plugin::deno_nevermore::LogMessage>>>
    {
        use tokio_stream::StreamExt;

        let app = ctx.data::<ThreadSafeApplication>()?;
        let mut app_locked = app.write().await;
        Ok(
            tokio_stream::wrappers::BroadcastStream::new(app_locked.subscribe_to_log())
                .map(|x| Ok(x?)),
        )
    }
}
