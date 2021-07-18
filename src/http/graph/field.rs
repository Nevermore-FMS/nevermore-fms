use async_graphql::*;
use async_graphql::guard::Guard;

use crate::application::ThreadSafeApplication;
use crate::field::driverstation::{ConfirmedState, State};
use crate::field::enums::{AllianceStation, DriverstationStatus, Mode};
use crate::http::graph::guards::UserTypeGuard;
use crate::models::user::UserType;

#[derive(SimpleObject)]
pub struct TeamAllianceStation {
    team_number: u16,
    alliance_station: AllianceStation
}

#[derive(InputObject)]
pub struct StateInput {
    pub emergency_stop: bool,
    pub enable: bool,

    pub mode: Mode,

    pub alliance_station: AllianceStation,
    pub status: DriverstationStatus,
    pub sequence_number: u16,
    pub time_to_display: u16,
    pub match_number: u16,
    pub event_name: String,
}

#[derive(Default)]
pub struct FieldQuery;

#[Object]
impl FieldQuery {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Viewer")))]
    async fn robot_state<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        team_number: u16,
    ) -> Result<State> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        Ok(locked_field.get_driver_station(team_number).await?.read().await.get_state().await?)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Viewer")))]
    async fn robot_confirmed_state<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        team_number: u16,
    ) -> Result<ConfirmedState> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        Ok(locked_field.get_driver_station(team_number).await?.read().await.get_confirmed_state()?)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Viewer")))]
    async fn team_alliance_stations<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<TeamAllianceStation>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        let team_alliance_stations = locked_field.get_team_alliance_station_map().await?.iter().map(|(team_number, alliance_station)| {
            TeamAllianceStation{
                team_number: *team_number,
                alliance_station: *alliance_station
            }
        }).collect();
        Ok(team_alliance_stations)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Viewer")))]
    async fn connected_team_numbers<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<Vec<u16>> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        Ok(locked_field.driver_station_team_numbers().await?)
    }
}

#[derive(Default)]
pub struct FieldMutation;

#[Object]
impl FieldMutation {
    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn add_team_to_field<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        team_number: u16,
        alliance_station: AllianceStation,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        locked_field.add_team(team_number, alliance_station).await?;
        Ok(true)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Admin")))]
    async fn remove_team_from_field<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        team_number: u16,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        locked_field.remove_team(team_number).await?;
        Ok(true)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Referee")))]
    async fn set_team_state<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        team_number: u16,
        state_input: StateInput
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        let state = State{
            emergency_stop: state_input.emergency_stop,
            enable: state_input.enable,
            mode: state_input.mode,
            team_number,
            alliance_station: state_input.alliance_station,
            status: state_input.status,
            sequence_number: state_input.sequence_number,
            time_to_display: state_input.time_to_display,
            match_number: state_input.match_number,
            event_name: state_input.event_name,
        };
        locked_field.get_driver_station(team_number).await?.write().await.set_state(state).await;
        Ok(true)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Referee")))]
    async fn set_all_teams_enabled<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        enabled: bool,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        locked_field.set_enabled_all(enabled).await?;
        Ok(true)
    }

    #[graphql(guard(UserTypeGuard(user_type = "UserType::Referee")))]
    async fn set_all_teams_emergency_stopped<'ctx>(
        &self,
        ctx: &Context<'ctx>,
        emergency_stop: bool,
    ) -> Result<bool> {
        let app = ctx.data::<ThreadSafeApplication>()?;
        let app_locked = app.read().await;
        let locked_field = app_locked.field.read().await;
        locked_field.set_emergency_stop_all(emergency_stop).await?;
        Ok(true)
    }
}

#[derive(Default)]
pub struct FieldSubscription;

#[Subscription]
impl FieldSubscription {
    async fn field_tick<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<impl tokio_stream::Stream<Item = Result<bool>>>
    {
        use tokio_stream::StreamExt;

        let reciever = {
            let app = ctx.data::<ThreadSafeApplication>()?;
            let app_locked = app.read().await;
            let field_locked = app_locked.field.read().await;
            field_locked.subscribe_to_tick_channel()?
        };
        Ok(
            tokio_stream::wrappers::BroadcastStream::new(reciever)
                .map(|_| Ok(true)),
        )
    }

    async fn field_close<'ctx>(
        &self,
        ctx: &Context<'ctx>,
    ) -> Result<impl tokio_stream::Stream<Item = Result<bool>>>
    {
        use tokio_stream::StreamExt;

        let reciever = {
            let app = ctx.data::<ThreadSafeApplication>()?;
            let app_locked = app.read().await;
            let field_locked = app_locked.field.read().await;
            field_locked.subscribe_to_close_channel()?
        };
        Ok(
            tokio_stream::wrappers::BroadcastStream::new(reciever)
                .map(|_| Ok(true)),
        )
    }
}