use crate::alarms::FMSAlarm;
use crate::field::Field;
use crate::graph::types::*;
use async_graphql::*;

pub struct GQLFieldState;

#[allow(unreachable_code)]
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

    async fn time_left(&self, ctx: &Context<'_>) -> f64 {
        let field = ctx.data::<Field>().unwrap();
        field.timer().await.current_time_remaining().as_secs_f64()
    }

    async fn udp_online(&self, ctx: &Context<'_>) -> bool {
        let field = ctx.data::<Field>().unwrap();
        field.udp_online().await
    }

    async fn tcp_online(&self, ctx: &Context<'_>) -> bool {
        let field = ctx.data::<Field>().unwrap();
        field.tcp_online().await
    }

    #[graphql(name = "activeFMSAlarms")]
    async fn active_fms_alarms(&self, ctx: &Context<'_>) -> Vec<GQLFMSAlarm> {
        let field = ctx.data::<Field>().unwrap();
        field
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
    async fn historic_fms_alarms(&self, ctx: &Context<'_>) -> Vec<GQLFMSAlarm> {
        let field = ctx.data::<Field>().unwrap();
        field
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
