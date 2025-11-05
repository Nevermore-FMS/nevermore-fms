use async_graphql::*;

use crate::graph::types::*;

#[derive(OneofObject)]
#[graphql(input_name = "DriverStationByCriteria")]
pub enum GQLDriverStationByCriteria {
    TeamNumber(u16),
    AllianceStation(GQLAllianceStation)
}