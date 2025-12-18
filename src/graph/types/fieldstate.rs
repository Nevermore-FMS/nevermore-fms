use crate::alarms::FMSAlarm;
use crate::field::Field;
use crate::graph::types::*;
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

pub struct GQLFMSAlarm {
    pub obj_fmsalarm: FMSAlarm,
}

#[Object(name = "FMSAlarm")]
impl GQLFMSAlarm {
    async fn id(&self) -> String {
        self.obj_fmsalarm.id.clone()
    }

    async fn alarm_type(&self) -> GQLFMSAlarmType {
        self.obj_fmsalarm.alarm_type.into()
    }

    async fn code(&self) -> String {
        self.obj_fmsalarm.code.clone()
    }

    async fn description(&self) -> String {
        self.obj_fmsalarm.description.clone()
    }

    async fn source_id(&self) -> String {
        self.obj_fmsalarm.source_id.clone()
    }

    async fn target_scope(&self) -> String {
        self.obj_fmsalarm.target_scope.clone()
    }

    async fn timestamp(&self) -> u64 {
        self.obj_fmsalarm.timestamp
    }

    async fn released(&self) -> bool {
        self.obj_fmsalarm.released
    }

    async fn auto_clear(&self) -> bool {
        self.obj_fmsalarm.auto_clear
    }
}
