use async_graphql::*;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(
    remote = "crate::field::enums::TournamentLevel",
    name = "TournamentLevel"
)]
pub enum GQLTournamentLevel {
    Test,
    Practice,
    Qualification,
    Playoff,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(remote = "crate::field::enums::Mode", name = "Mode")]
pub enum GQLMode {
    TeleOp,
    Test,
    Autonomous,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(
    remote = "crate::field::enums::AllianceStation",
    name = "AllianceStation"
)]
pub enum GQLAllianceStation {
    Red1,
    Red2,
    Red3,
    Blue1,
    Blue2,
    Blue3,
    None,
}
