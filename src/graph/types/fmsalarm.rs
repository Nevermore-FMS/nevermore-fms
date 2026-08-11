use async_graphql::Object;

use crate::{alarms::FMSAlarm, graph::types::GQLFMSAlarmType};

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
