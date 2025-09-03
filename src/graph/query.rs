use async_graphql::*;

use crate::{field::Field, graph::types::GQLDriverStation};

use super::types::GQLFieldState;

pub struct Query;

#[allow(unreachable_code)]
#[Object]
impl Query {
    async fn field_state(&self) -> GQLFieldState {
        GQLFieldState
    }

    async fn driver_stations(&self, ctx: &Context<'_>) -> Vec<GQLDriverStation> {
        let field = ctx.data::<Field>().unwrap();
        let mut dss = Vec::new();
        dss.push(GQLDriverStation {
            obj_driverstation: field
                .driverstations()
                .await
                .get_driverstation_by_position(crate::field::enums::AllianceStation::Red1)
                .await
                .unwrap(),
        });
        dss
    }
}
