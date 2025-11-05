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
        self.obj_field.event_name().await
    }

    async fn tournament_level(&self) -> GQLTournamentLevel {
        GQLTournamentLevel::from(self.obj_field.tournament_level().await)
    }

    async fn match_number(&self) -> u16 {
        self.obj_field.match_number().await
    }

    async fn play_number(&self) -> u8 {
        self.obj_field.play_number().await
    }

    async fn time_left(&self) -> f64 {
        self.obj_field
            .timer()
            .await
            .current_time_remaining()
            .as_secs_f64()
    }

    async fn udp_online(&self) -> bool {
        self.obj_field.udp_online().await
    }

    async fn tcp_online(&self) -> bool {
        self.obj_field.tcp_online().await
    }

    #[graphql(name = "activeFMSAlarms")]
    async fn active_fms_alarms(&self) -> Vec<GQLFMSAlarm> {
        self.obj_field
            .alarm_handler()
            .await
            .active_alarms()
            .await
            .iter()
            .cloned()
            .map(|alarm| GQLFMSAlarm {
                obj_fmsalarm: alarm,
            })
            .collect()
    }

    #[graphql(name = "historicFMSAlarms")]
    async fn historic_fms_alarms(&self) -> Vec<GQLFMSAlarm> {
        self.obj_field
            .alarm_handler()
            .await
            .historic_alarms()
            .await
            .iter()
            .cloned()
            .map(|alarm| GQLFMSAlarm {
                obj_fmsalarm: alarm,
            })
            .collect()
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
        GQLFMSAlarmType::from(self.obj_fmsalarm.alarm_type)
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
