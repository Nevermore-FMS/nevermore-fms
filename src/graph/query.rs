#![allow(clippy::unused_async)]

use async_graphql::*;

use crate::fmscore::FMSCore;
use crate::graph::error::internalize_err;
use crate::graph::inputs::*;
use crate::graph::types::*;

pub struct Query;

#[allow(unreachable_code)]
#[Object]
impl Query {
    //TODO Auth

    async fn users(&self, ctx: &Context<'_>) -> anyhow::Result<Vec<GQLUser>> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        Ok(fms_core
            .main_db()
            .get_users().map_err(internalize_err)?
            .iter()
            .cloned()
            .map(|user| GQLUser { obj_user: user })
            .collect())
    }

    async fn field_state(&self, ctx: &Context<'_>) -> GQLFieldState {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        GQLFieldState {
            obj_field: fms_core.field(),
        }
    }

    #[graphql(name = "activeFMSAlarms")]
    async fn active_fms_alarms(&self, ctx: &Context<'_>) -> Vec<GQLFMSAlarm> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        fms_core
            .field()
            .alarm_handler()
            .active_alarms()
            .iter()
            .cloned()
            .map(|alarm| GQLFMSAlarm {
                obj_fmsalarm: alarm,
            })
            .collect()
    }

    #[graphql(name = "historicFMSAlarms")]
    async fn historic_fms_alarms(&self, ctx: &Context<'_>) -> Vec<GQLFMSAlarm> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        fms_core
            .field()
            .alarm_handler()
            .historic_alarms()
            .iter()
            .cloned()
            .map(|alarm| GQLFMSAlarm {
                obj_fmsalarm: alarm,
            })
            .collect()
    }

    async fn driver_stations(&self, ctx: &Context<'_>) -> Vec<GQLDriverStation> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        fms_core
            .field()
            .driverstations()
            .get_all_driverstations()
            .iter()
            .map(|ds| GQLDriverStation {
                obj_driverstation: ds.clone(),
            })
            .collect()
    }

    async fn driver_station(
        &self,
        ctx: &Context<'_>,
        criteria: GQLDriverStationByCriteriaInput,
    ) -> Option<GQLDriverStation> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        match criteria {
            GQLDriverStationByCriteriaInput::AllianceStation(alliance_station) => fms_core
                .field()
                .driverstations()
                .get_driverstation_by_position(alliance_station.into())
                .map(|ds| GQLDriverStation {
                    obj_driverstation: ds,
                }),
            GQLDriverStationByCriteriaInput::TeamNumber(team_number) => fms_core
                .field()
                .driverstations()
                .get_driverstation_by_team_number(team_number)
                .map(|ds| GQLDriverStation {
                    obj_driverstation: ds,
                }),
        }
    }

    async fn current_match(&self, ctx: &Context<'_>) -> Option<GQLFieldMatch> {
        let _fms_core = ctx.data::<FMSCore>().unwrap();
        None
    }
}
