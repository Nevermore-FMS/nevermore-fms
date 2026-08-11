use async_graphql::*;

use crate::graph::types::*;


#[derive(InputObject)]
#[graphql(input_name = "NewDsInput")]
pub struct GQLNewDsInput {
    pub team_number: u16,
    pub alliance_station: GQLAllianceStation
}