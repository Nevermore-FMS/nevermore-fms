#![allow(clippy::unused_async)]

use anyhow::bail;
use async_graphql::*;

use crate::authentication::nevermore_fms_permissions;
use crate::fmscore::FMSCore;
use crate::web::graph::guard::PermissionGuard;
use crate::web::graph::inputs::*;
use crate::web::graph::types::*;

pub struct Mutation;

#[allow(unreachable_code)]
#[Object]
impl Mutation {
    #[graphql(
        name = "clearFMSAlarm",
        guard = "PermissionGuard::requires(nevermore_fms_permissions::CommandField)"
    )]
    async fn clear_fms_alarm(&self, ctx: &Context<'_>, code: String) -> anyhow::Result<bool> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        fms_core.field().alarm_handler().clear_alarm(&code)
    }

    #[graphql(
        name = "clearAllFMSAlarms",
        guard = "PermissionGuard::requires(nevermore_fms_permissions::CommandField)"
    )]
    async fn clear_all_fms_alarms(&self, ctx: &Context<'_>) -> anyhow::Result<bool> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        fms_core.field().alarm_handler().clear_all_alarms()
    }

    #[graphql(
        name = "setDS",
        guard = "PermissionGuard::requires(nevermore_fms_permissions::CommandField)"
    )]
    async fn set_ds(
        &self,
        ctx: &Context<'_>,
        new_driver_stations: Vec<GQLNewDsInput>,
    ) -> anyhow::Result<Vec<GQLDriverStation>> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        let driverstations = fms_core.field().driverstations();
        let mut added_dss = Vec::new();
        for new_ds in new_driver_stations {
            if let Some(existing_ds) =
                driverstations.get_driverstation_by_position(new_ds.alliance_station.into())
            {
                driverstations
                    .delete_driverstation(existing_ds.team_number())
                    .await?;
            }

            if let Some(existing_ds) =
                driverstations.get_driverstation_by_team_number(new_ds.team_number)
            {
                driverstations
                    .delete_driverstation(existing_ds.team_number())
                    .await?;
            }

            let added_ds = driverstations
                .add_driverstation(new_ds.team_number, new_ds.alliance_station.into())?;
            added_dss.push(GQLDriverStation {
                obj_driverstation: added_ds,
            });
        }

        Ok(added_dss)
    }

    #[graphql(
        name = "removeDS",
        guard = "PermissionGuard::requires(nevermore_fms_permissions::CommandField)"
    )]
    async fn remove_ds(
        &self,
        ctx: &Context<'_>,
        criteria: GQLDriverStationByCriteriaInput,
    ) -> anyhow::Result<bool> {
        let fms_core = ctx.data::<FMSCore>().unwrap();
        let driverstations = fms_core.field().driverstations();
        let current_ds = match criteria {
            GQLDriverStationByCriteriaInput::AllianceStation(alliance_station) => {
                driverstations.get_driverstation_by_position(alliance_station.into())
            }
            GQLDriverStationByCriteriaInput::TeamNumber(team_number) => {
                driverstations.get_driverstation_by_team_number(team_number)
            }
        };
        if let Some(ds) = current_ds {
            fms_core
                .field()
                .driverstations()
                .delete_driverstation(ds.team_number())
                .await?;
            Ok(true)
        } else {
            bail!("DriverStation does not exist")
        }
    }
}
