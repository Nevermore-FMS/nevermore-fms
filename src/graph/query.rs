use async_graphql::*;

use crate::field::Field;
use crate::graph::inputs::*;
use crate::graph::types::*;

pub struct Query;

#[allow(unreachable_code)]
#[Object]
impl Query {
    //TODO Auth


    async fn field_state(&self, ctx: &Context<'_>) -> GQLFieldState {
        let field = ctx.data::<Field>().unwrap();
        GQLFieldState {
            obj_field: field.to_owned(),
        }
    }

        #[graphql(name = "activeFMSAlarms")]
    async fn active_fms_alarms(&self, ctx: &Context<'_>) -> Vec<GQLFMSAlarm> {
        let field = ctx.data::<Field>().unwrap();
        field
            .alarm_handler()
            .await
            .active_alarms()
            .await
            .iter()
            .cloned()
            .map(|alarm| GQLFMSAlarm {
                obj_fmsalarm: alarm,
            })
            .collect()
    }

    #[graphql(name = "historicFMSAlarms")]
    async fn historic_fms_alarms(&self, ctx: &Context<'_>) -> Vec<GQLFMSAlarm> {
        let field = ctx.data::<Field>().unwrap();
        field
            .alarm_handler()
            .await
            .historic_alarms()
            .await
            .iter()
            .cloned()
            .map(|alarm| GQLFMSAlarm {
                obj_fmsalarm: alarm,
            })
            .collect()
    }

    async fn driver_stations(&self, ctx: &Context<'_>) -> Vec<GQLDriverStation> {
        let field = ctx.data::<Field>().unwrap();
        field
            .driverstations()
            .await
            .get_all_driverstations()
            .await
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
        let field = ctx.data::<Field>().unwrap();
        match criteria {
            GQLDriverStationByCriteriaInput::AllianceStation(alliance_station) => field
                .driverstations()
                .await
                .get_driverstation_by_position(alliance_station.into())
                .await
                .map(|ds| GQLDriverStation {
                    obj_driverstation: ds,
                }),
            GQLDriverStationByCriteriaInput::TeamNumber(team_number) => field
                .driverstations()
                .await
                .get_driverstation_by_team_number(team_number.into())
                .await
                .map(|ds| GQLDriverStation {
                    obj_driverstation: ds,
                }),
        }
    }

    async fn current_match(&self, ctx: &Context<'_>) -> Option<GQLFieldMatch> {
        let _field = ctx.data::<Field>().unwrap();
        None
    }
}
