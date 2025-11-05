use async_graphql::*;

use crate::field::Field;
use crate::graph::types::*;

pub struct Query;

#[allow(unreachable_code)]
#[Object]
impl Query {
    async fn field_state(&self, ctx: &Context<'_>) -> GQLFieldState {
        let field = ctx.data::<Field>().unwrap();
        GQLFieldState {
            obj_field: field.to_owned()
        }
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
}
