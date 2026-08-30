use crate::field::Field;
use crate::web::graph::types::*;
use async_graphql::*;

pub struct GQLFieldState {
    pub obj_field: Field,
}

#[allow(unreachable_code)]
#[Object(name = "FieldState")]
impl GQLFieldState {
    async fn event_name(&self) -> String {
        self.obj_field.event_name()
    }

    async fn tournament_level(&self) -> GQLTournamentLevel {
        self.obj_field.tournament_level().into()
    }

    async fn match_number(&self) -> u16 {
        self.obj_field.match_number()
    }

    async fn play_number(&self) -> u8 {
        self.obj_field.play_number()
    }

    async fn time_left(&self) -> f64 {
        self.obj_field
            .timer()
            .current_time_remaining()
            .as_secs_f64()
    }

    async fn ds_mode(&self) -> GQLMode {
        self.obj_field.ds_mode().into()
    }

    async fn is_safe(&self) -> bool {
        self.obj_field.is_safe()
    }

    async fn udp_online(&self) -> bool {
        self.obj_field.udp_online()
    }

    async fn tcp_online(&self) -> bool {
        self.obj_field.tcp_online()
    }
}