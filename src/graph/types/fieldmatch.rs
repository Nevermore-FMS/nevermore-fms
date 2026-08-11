use async_graphql::*;

use crate::graph::types::GQLAllianceStation;

pub struct GQLFieldMatch {
    //pub obj_match: DriverStation,
}

#[Object(name = "FieldMatch")]
impl GQLFieldMatch {
    async fn id(&self) -> ID {
        ID("TODO".into())
    }

    async fn scheduled_match(&self) -> u64 {
        0
    }

    async fn play_number(&self) -> u64 {
        0
    }

    async fn bypassed_alliance_stations(&self) -> Vec<GQLAllianceStation> {
        Vec::new()
    }

    async fn red_score(&self) -> u64 {
        0
    }

    async fn blue_score(&self) -> u64 {
        0
    }

    async fn started_at_timestamp(&self) -> u64 {
        0
    }

    async fn completed(&self) -> bool {
        false
    }

    async fn completed_successfully(&self) -> bool {
        false
    }
}