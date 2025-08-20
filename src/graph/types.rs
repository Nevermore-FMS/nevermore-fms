use async_graphql::{Enum, SimpleObject};

use crate::field::enums::TournamentLevel;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::field::enums::TournamentLevel")]
pub enum GQLTournamentLevel {
    Test,
    Practice,
    Qualification,
    Playoff,
}


#[derive(SimpleObject)]
pub struct FieldState {
    pub event_name: String,
    pub tournament_level: GQLTournamentLevel,
    pub match_number: u32,
    pub play_number: u32,
    pub udp_online: bool,
    pub tcp_online: bool
}