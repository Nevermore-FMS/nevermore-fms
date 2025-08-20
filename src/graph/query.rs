use async_graphql::*;

use super::types::{FieldState, GQLTournamentLevel};

pub struct Query;

#[Object]
impl Query {
    async fn field_state(&self) -> FieldState {
        return FieldState {
            event_name: "Test".to_string(),
            tournament_level: GQLTournamentLevel::Practice,
            match_number: 1,
            play_number: 1,
            udp_online: true,
            tcp_online: true
        }
    }
}