use anyhow::bail;
use async_graphql::*;

use crate::field::Field;
use crate::graph::inputs::*;
use crate::graph::types::*;

pub struct Mutation;

#[allow(unreachable_code)]
#[Object]
impl Mutation {
    //TODO Auth

    #[graphql(name = "clearFMSAlarm")]
    async fn clear_fms_alarm(&self, ctx: &Context<'_>, code: String) -> anyhow::Result<bool> {
        let field = ctx.data::<Field>().unwrap();
        return field.alarm_handler().await.clear_alarm(&code).await;
    }

    #[graphql(name = "clearAllFMSAlarms")]
    async fn clear_all_fms_alarms(&self, ctx: &Context<'_>) -> anyhow::Result<bool> {
        let field = ctx.data::<Field>().unwrap();
        return field.alarm_handler().await.clear_all_alarms().await;
    }

    #[graphql(name = "setDS")]
    async fn set_ds(
        &self,
        ctx: &Context<'_>,
        new_driver_stations: Vec<GQLNewDsInput>,
    ) -> anyhow::Result<Vec<GQLDriverStation>> {
        let field = ctx.data::<Field>().unwrap();
        let driverstations = field.driverstations().await;
        let mut added_dss = Vec::new();
        for new_ds in new_driver_stations {
            if let Some(existing_ds) = driverstations
                .get_driverstation_by_position(new_ds.alliance_station.into())
                .await
            {
                driverstations
                    .delete_driverstation(existing_ds.team_number().await)
                    .await?;
            }

            if let Some(existing_ds) = driverstations
                .get_driverstation_by_team_number(new_ds.team_number)
                .await
            {
                driverstations
                    .delete_driverstation(existing_ds.team_number().await)
                    .await?;
            }

            let added_ds = driverstations
                .add_driverstation(new_ds.team_number, new_ds.alliance_station.into())
                .await?;
            added_dss.push(GQLDriverStation {
                obj_driverstation: added_ds,
            });
        }

        Ok(added_dss)
    }

    #[graphql(name = "removeDS")]
    async fn remove_ds(
        &self,
        ctx: &Context<'_>,
        criteria: GQLDriverStationByCriteriaInput,
    ) -> anyhow::Result<bool> {
        let field = ctx.data::<Field>().unwrap();
        let driverstations = field.driverstations().await;
        let current_ds = match criteria {
            GQLDriverStationByCriteriaInput::AllianceStation(alliance_station) => {
                driverstations.get_driverstation_by_position(alliance_station.into()).await
            },
            GQLDriverStationByCriteriaInput::TeamNumber(team_number) => {
                driverstations.get_driverstation_by_team_number(team_number).await
            }
        };
        if let Some(ds) = current_ds {
            field.driverstations().await.delete_driverstation(ds.team_number().await).await?;
            Ok(true)
        } else {
            bail!("DriverStation does not exist")
        }
    }
}
