use async_graphql::*;
use async_graphql::guard::Guard;

use crate::application::ThreadSafeApplication;
use crate::models::config::{Config, ConfigKey};
use crate::http::graph::guards::UserTypeGuard;
use crate::models::user::UserType;

#[derive(Default)]
pub struct ConfigQuery;

#[Object]
impl ConfigQuery {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn config_entry<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        key: ConfigKey
    ) -> Result<Option<String>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        Ok(Config::get(app_locked.database.clone(), key).await)
    }
}

#[derive(Default)]
pub struct ConfigMutation;

#[Object]
impl ConfigMutation {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn update_config_entry<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        key: ConfigKey,
        value: String
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        Config::set(app_locked.database.clone(), key, value).await?;
        Ok(true)
    }
}
