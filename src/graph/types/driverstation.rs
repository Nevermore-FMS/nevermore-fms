use crate::field::connection::DriverStationConnection;
use crate::field::driverstation::{
    DriverStation, DriverStationConfirmedState, DriverStationLogData, DriverStationLogMessage,
};
use crate::field::enums::VersionData;
use crate::graph::types::*;
use async_graphql::*;

pub struct GQLDriverStation {
    pub obj_driverstation: DriverStation,
}

#[Object(name = "DriverStation")]
impl GQLDriverStation {
    async fn team_number(&self) -> u16 {
        self.obj_driverstation.team_number().await
    }

    async fn alliance_station(&self) -> GQLAllianceStation {
        self.obj_driverstation.alliance_station().await.into()
    }

    async fn commanded_enabled(&self) -> bool {
        self.obj_driverstation.commanded_enabled().await
    }

    async fn enabled(&self) -> bool {
        self.obj_driverstation.enabled().await
    }

    async fn expected_ip(&self) -> Option<GQLIpCidr> {
        match self.obj_driverstation.expected_ip().await {
            None => None,
            Some(expected_ip) => Some(GQLIpCidr(expected_ip)),
        }
    }

    async fn active_connection(&self) -> Option<GQLDriverStationConnection> {
        match self.obj_driverstation.active_connection().await {
            Some(x) => Some(GQLDriverStationConnection {
                obj_driverstationconnection: x,
            }),
            None => None,
        }
    }

    async fn confirmed_state(&self) -> Option<GQLDriverStationConfirmedState> {
        match self.obj_driverstation.confirmed_state().await {
            Some(x) => Some(GQLDriverStationConfirmedState {
                obj_driverstationconfirmedstate: x,
            }),
            None => None,
        }
    }

    async fn log_data(&self) -> Vec<GQLDriverStationLogData> {
        self.obj_driverstation
            .log_data()
            .await
            .iter()
            .map(|log_data| GQLDriverStationLogData {
                obj_driverstationlogdata: log_data.clone(),
            })
            .collect()
    }

    async fn log_messages(&self) -> Vec<GQLDriverStationLogMessage> {
        self.obj_driverstation
            .log_messages()
            .await
            .iter()
            .map(|log_message| GQLDriverStationLogMessage {
                obj_driverstationlogmessage: log_message.clone(),
            })
            .collect()
    }

    async fn versions(&self) -> Vec<GQLVersionData> {
        self.obj_driverstation
            .versions()
            .await
            .values()
            .cloned()
            .map(|version_data| GQLVersionData {
                obj_versiondata: version_data,
            })
            .collect()
    }
}

pub struct GQLDriverStationConnection {
    pub obj_driverstationconnection: DriverStationConnection,
}

#[Object(name = "DriverStationConnection")]
impl GQLDriverStationConnection {
    async fn id(&self) -> ID {
        ID(self.obj_driverstationconnection.uuid().await.to_string())
    }

    async fn is_alive(&self) -> bool {
        self.obj_driverstationconnection.is_alive().await
    }

    async fn ip_address(&self) -> GQLIpAddr {
        GQLIpAddr(self.obj_driverstationconnection.ip_address().await)
    }

    async fn last_packet_recieved_at_millis(&self) -> i64 {
        self.obj_driverstationconnection
            .last_udp_packet_reception()
            .await
            .timestamp_millis()
    }
}

pub struct GQLDriverStationConfirmedState {
    pub obj_driverstationconfirmedstate: DriverStationConfirmedState,
}

#[Object(name = "DriverStationConfirmedState")]
impl GQLDriverStationConfirmedState {
    async fn is_emergency_stopped(&self) -> bool {
        self.obj_driverstationconfirmedstate.is_emergency_stopped
    }

    async fn robot_communications_active(&self) -> bool {
        self.obj_driverstationconfirmedstate
            .robot_communications_active
    }

    async fn can_ping_radio(&self) -> bool {
        self.obj_driverstationconfirmedstate.can_ping_radio
    }

    async fn can_ping_rio(&self) -> bool {
        self.obj_driverstationconfirmedstate.can_ping_rio
    }

    async fn is_enabled(&self) -> bool {
        self.obj_driverstationconfirmedstate.is_enabled
    }

    async fn mode(&self) -> GQLMode {
        self.obj_driverstationconfirmedstate.mode.into()
    }

    async fn team_number(&self) -> u16 {
        self.obj_driverstationconfirmedstate.team_number
    }

    async fn battery_voltage(&self) -> f32 {
        self.obj_driverstationconfirmedstate.battery_voltage
    }
}

pub struct GQLDriverStationLogData {
    pub obj_driverstationlogdata: DriverStationLogData,
}

#[Object(name = "DriverStationLogData")]
impl GQLDriverStationLogData {
    async fn timestamp(&self) -> u64 {
        self.obj_driverstationlogdata.timestamp
    }

    async fn trip_time(&self) -> u8 {
        self.obj_driverstationlogdata.trip_time
    }

    async fn lost_packets(&self) -> u8 {
        self.obj_driverstationlogdata.lost_packets
    }

    async fn voltage(&self) -> f32 {
        self.obj_driverstationlogdata.voltage
    }

    async fn brownout(&self) -> bool {
        self.obj_driverstationlogdata.brownout
    }

    async fn watchdog(&self) -> bool {
        self.obj_driverstationlogdata.watchdog
    }
    async fn ds_teleop(&self) -> bool {
        self.obj_driverstationlogdata.ds_teleop
    }

    async fn ds_auto(&self) -> bool {
        self.obj_driverstationlogdata.ds_auto
    }

    async fn ds_disable(&self) -> bool {
        self.obj_driverstationlogdata.ds_disable
    }

    async fn robot_teleop(&self) -> bool {
        self.obj_driverstationlogdata.robot_teleop
    }

    async fn robot_auto(&self) -> bool {
        self.obj_driverstationlogdata.robot_auto
    }
    async fn robot_disable(&self) -> bool {
        self.obj_driverstationlogdata.robot_disable
    }

    async fn can_utilization(&self) -> u8 {
        self.obj_driverstationlogdata.can_utilization
    }

    async fn signal(&self) -> u8 {
        self.obj_driverstationlogdata.signal
    }

    async fn bandwidth(&self) -> f32 {
        self.obj_driverstationlogdata.bandwidth
    }
}

pub struct GQLDriverStationLogMessage {
    pub obj_driverstationlogmessage: DriverStationLogMessage,
}

#[Object(name = "DriverStationLogMessage")]
impl GQLDriverStationLogMessage {
    async fn timestamp(&self) -> u64 {
        self.obj_driverstationlogmessage.timestamp
    }

    async fn local_timestamp(&self) -> u64 {
        self.obj_driverstationlogmessage.local_timestamp
    }

    async fn message(&self) -> String {
        self.obj_driverstationlogmessage.message.clone()
    }
}

pub struct GQLVersionData {
    pub obj_versiondata: VersionData,
}

#[Object(name = "VersionData")]
impl GQLVersionData {
    async fn version_type(&self) -> GQLVersionType {
        self.obj_versiondata.version_type.into()
    }

    async fn status(&self) -> String {
        self.obj_versiondata.status.clone()
    }

    async fn version(&self) -> String {
        self.obj_versiondata.version.clone()
    }
}
