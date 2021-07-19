use async_graphql::guard::Guard;
use async_graphql::*;

use crate::application::ThreadSafeApplication;
use crate::field::network::{AllianceStationToConfiguration, NetworkConfiguratorInfo, Reply};
use crate::http::graph::guards::UserTypeGuard;
use crate::models::config::{Config, ConfigKey};
use crate::models::user::UserType;

#[derive(Default)]
pub struct NetworkQuery;

#[Object]
impl NetworkQuery {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn network_configurator_all_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<NetworkConfiguratorInfo>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        let configrator_map = locked_field.network_configurator_map();
        let configurator_map_locked = configrator_map.read().await;
        Ok(configurator_map_locked.get_all_info().await)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn network_configurator_info<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        name: String,
    ) -> Result<NetworkConfiguratorInfo> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        let configrator_map = locked_field.network_configurator_map();
        let configurator_map_locked = configrator_map.read().await;
        Ok(configurator_map_locked.get_info(name).await?)
    }
}

#[derive(Default)]
pub struct NetworkMutation;

#[Object]
impl NetworkMutation {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn network_scan<'ctx>(&self, ctx: &Context<'ctx>) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        let network_name = Config::get(
            app_locked.database.clone(),
            ConfigKey::ActiveNetworkConfigurator,
        )
        .await
        .ok_or("No Active Network Configurator.")?;
        let configrator_map = locked_field.network_configurator_map();
        let configurator_map_locked = configrator_map.read().await;
        let locked_network = configurator_map_locked
            .get(network_name)
            .ok_or("No Network found for name.")?;
        let reply = locked_network.read().await.run_scan().await?;
        match reply {
            Reply::SUCCESS => Ok(true),
            Reply::ERROR(error) => Err(error.into()),
        }
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn network_initial_configuration<'ctx>(&self, ctx: &Context<'ctx>) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        let network_name = Config::get(
            app_locked.database.clone(),
            ConfigKey::ActiveNetworkConfigurator,
        )
        .await
        .ok_or("No Active Network Configurator.")?;
        let configrator_map = locked_field.network_configurator_map();
        let configurator_map_locked = configrator_map.read().await;
        let locked_network = configurator_map_locked
            .get(network_name)
            .ok_or("No Network found for name.")?;
        let reply = locked_network
            .read()
            .await
            .run_initial_configuration()
            .await?;
        match reply {
            Reply::SUCCESS => Ok(true),
            Reply::ERROR(error) => Err(error.into()),
        }
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn network_match_configuration<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        station_config: AllianceStationToConfiguration,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        let network_name = Config::get(
            app_locked.database.clone(),
            ConfigKey::ActiveNetworkConfigurator,
        )
        .await
        .ok_or("No Active Network Configurator.")?;
        let configrator_map = locked_field.network_configurator_map();
        let configurator_map_locked = configrator_map.read().await;
        let locked_network = configurator_map_locked
            .get(network_name)
            .ok_or("No Network found for name.")?;
        let reply = locked_network
            .read()
            .await
            .run_match_configuration(station_config)
            .await?;
        match reply {
            Reply::SUCCESS => Ok(true),
            Reply::ERROR(error) => Err(error.into()),
        }
    }
}
