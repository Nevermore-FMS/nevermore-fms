use async_graphql::*;
use cidr::AnyIpCidr;

use crate::field::{Field, driverstation::DriverStation};

struct GQLIpCidr(AnyIpCidr);

#[Scalar(name = "IpCidr")]
impl ScalarType for GQLIpCidr {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = &value {
            // Parse the integer value
            Ok(value.parse().map(GQLIpCidr)?)
        } else {
            // If the type does not match
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}

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

pub struct GQLFieldState;

#[Object(name = "FieldState")]
impl GQLFieldState {
    async fn event_name(&self, ctx: &Context<'_>) -> String {
        let field = ctx.data::<Field>().unwrap();
        field.event_name().await
    }

    async fn tournament_level(&self, ctx: &Context<'_>) -> GQLTournamentLevel {
        let field = ctx.data::<Field>().unwrap();
        GQLTournamentLevel::from(field.tournament_level().await)
    }

    async fn match_number(&self, ctx: &Context<'_>) -> u16 {
        let field = ctx.data::<Field>().unwrap();
        field.match_number().await
    }

    async fn play_number(&self, ctx: &Context<'_>) -> u8 {
        let field = ctx.data::<Field>().unwrap();
        field.play_number().await
    }

    async fn udp_online(&self, ctx: &Context<'_>) -> bool {
        let field = ctx.data::<Field>().unwrap();
        field.udp_online().await
    }

    async fn tcp_online(&self, ctx: &Context<'_>) -> bool {
        let field = ctx.data::<Field>().unwrap();
        field.tcp_online().await
    }
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

pub struct GQLDriverStation {
    pub obj_driverstation: DriverStation,
}

#[Object(name = "DriverStation")]
impl GQLDriverStation {
    async fn team_number(&self) -> u16 {
        self.obj_driverstation.team_number().await
    }

    async fn alliance_station(&self) -> GQLAllianceStation {
        GQLAllianceStation::from(self.obj_driverstation.alliance_station().await)
    }

    async fn expected_ip(&self) -> Option<GQLIpCidr> {
        match self.obj_driverstation.expected_ip().await {
            None => None,
            Some(expected_ip) => Some(GQLIpCidr(expected_ip)),
        }
    }

    /*async fn active_connection(&self) -> u16 {
        self.obj_driverstation.team_number().await
    }*/

    /*async fn confirmed_state(&self) -> u16 {
        self.obj_driverstation.team_number().await
    }*/

    /*async fn log_data(&self) -> u16 {
        self.obj_driverstation.team_number().await
    }*/

    /*async fn versions(&self) -> u16 {
        self.obj_driverstation.team_number().await
    }*/

    /*async fn log_messages(&self) -> u16 {
        self.obj_driverstation.team_number().await
    }*/
}
