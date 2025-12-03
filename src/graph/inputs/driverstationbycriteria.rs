use async_graphql::*;

use crate::graph::types::*;

#[derive(OneofObject)]
#[graphql(input_name = "DriverStationByCriteriaInput")]
pub enum GQLDriverStationByCriteriaInput {
    TeamNumber(u16),
    AllianceStation(GQLAllianceStation)
}