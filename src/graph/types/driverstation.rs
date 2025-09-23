use crate::field::connection::DriverStationConnection;
use crate::field::driverstation::{DriverStation, DriverStationConfirmedState};
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
        GQLAllianceStation::from(self.obj_driverstation.alliance_station().await)
    }

    async fn expected_ip(&self) -> Option<GQLIpCidr> {
        match self.obj_driverstation.expected_ip().await {
            None => None,
            Some(expected_ip) => Some(GQLIpCidr(expected_ip)),
        }
    }

    async fn active_connection(&self) -> Option<GQLDriverStationConnection> {
        match self.obj_driverstation.active_connection().await {
            Some(x) => Some(GQLDriverStationConnection { obj_driverstationconnection: x }),
            None => None
        }
        
    }

    async fn confirmed_state(&self) -> Option<GQLDriverStationConfirmedState> {
        match self.obj_driverstation.confirmed_state().await {
            Some(x) => Some(GQLDriverStationConfirmedState { obj_driverstationconfirmedstate: x }),
            None => None
        }
    }

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
        self.obj_driverstationconnection.last_udp_packet_reception().await.timestamp_millis()
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
        self.obj_driverstationconfirmedstate.robot_communications_active
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
        GQLMode::from(self.obj_driverstationconfirmedstate.mode)
    }

    async fn team_number(&self) -> u16 {
        self.obj_driverstationconfirmedstate.team_number
    }

    async fn battery_voltage(&self) -> f32 {
        self.obj_driverstationconfirmedstate.battery_voltage
    }

}