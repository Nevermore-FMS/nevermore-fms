use std::time::Duration;

use crate::control::{enabler, estopper};
use crate::field::enums::{AllianceStation, Mode, TournamentLevel};
use crate::field::{driverstation, enums, Field};
use cidr::{AnyIpCidr, Ipv4Cidr};
use log::info;
use tokio::sync::{broadcast, mpsc};
use tokio_stream::wrappers::{BroadcastStream, ReceiverStream};
use tonic::{transport::Server, Request, Response, Status};

use super::rpc::enabler_config::Enabler;
use super::rpc::estopper_config::Estopper;
use super::rpc::generic_api_server::{GenericApi, GenericApiServer};
use super::PluginManager;

use super::rpc::network_configurator_api_server::NetworkConfiguratorApi;
use super::rpc::{
    DriverStation, DriverStationParams, DriverStationQuery, DriverStationQueryType,
    DriverStationUpdateExpectedIp, DriverStationUpdateMode, DriverStations, Empty, EnablerConfig,
    EnablerQuery, EstopperConfig, EstopperQuery, FieldConfiguration, FieldState, FieldTimerUpdate,
};

pub struct GenericApiImpl {
    pub plugin_manager: PluginManager,
    pub field: Field,
}

#[tonic::async_trait]
impl GenericApi for GenericApiImpl {
    type OnFieldStateUpdateStream = ReceiverStream<Result<FieldState, Status>>;

    async fn on_field_state_update(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnFieldStateUpdateStream>, Status> {
        let (tx, rx) = mpsc::channel::<Result<FieldState, Status>>(4);

        tokio::spawn(async move {
            loop {
                let res = tx.send(Ok(FieldState::default())).await;
                if res.is_err() {
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type OnFieldTerminateStream = ReceiverStream<Result<FieldState, Status>>;

    async fn on_field_terminate(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnFieldTerminateStream>, Status> {
        Err(Status::unknown("TODO"))
    }

    async fn get_field_state(&self, _: Request<Empty>) -> Result<Response<FieldState>, Status> {
        Ok(Response::new(self.field.state_to_rpc().await))
    }

    async fn configure_field(
        &self,
        request: Request<FieldConfiguration>,
    ) -> Result<Response<FieldState>, Status> {
        self.field
            .set_event_name(request.get_ref().event_name.clone())
            .await;
        self.field
            .set_tournament_level(TournamentLevel::from_byte(
                request.get_ref().tournament_level as u8,
            ))
            .await;
        self.field
            .set_match_number(request.get_ref().match_number as u16)
            .await;
        self.field
            .set_play_number(request.get_ref().play_number as u8)
            .await;

        Ok(Response::new(self.field.state_to_rpc().await))
    }

    async fn update_field_timer(
        &self,
        request: Request<FieldTimerUpdate>,
    ) -> Result<Response<FieldState>, Status> {
        if let Some(time_left) = request.get_ref().time_remaining {
            self.field
                .set_time_remaining(Duration::from_millis(time_left))
                .await;
        }

        if request.get_ref().running {
            self.field.start_timer().await;
        } else {
            self.field.stop_timer().await;
        }

        Ok(Response::new(self.field.state_to_rpc().await))
    }

    async fn update_enabler(
        &self,
        request: Request<EnablerConfig>,
    ) -> Result<Response<Empty>, Status> {
        let enabler: enabler::Enabler = match &request.get_ref().enabler {
            Some(Enabler::AllEnabler(config)) => {
                enabler::AllEnabler::new(request.get_ref().name.clone(), config.active)
            }
            Some(Enabler::AllianceStationEnabler(config)) => {
                let mut approved_stations: Vec<AllianceStation> = Vec::new();
                for station in config.approved_stations.iter() {
                    approved_stations.push(AllianceStation::from_byte(station.clone() as u8));
                }
                enabler::AllianceStationEnabler::new(
                    request.get_ref().name.clone(),
                    approved_stations,
                )
            }
            Some(Enabler::TeamNumberEnabler(config)) => {
                let mut approved_teams: Vec<u16> = Vec::new();
                for team in config.approved_team_numbers.iter() {
                    approved_teams.push(team.clone() as u16);
                }
                enabler::TeamNumberEnabler::new(request.get_ref().name.clone(), approved_teams)
            }
            _ => return Err(Status::invalid_argument("Configuration message not proper")),
        };
        self.field
            .control_system()
            .await
            .update_enabler(
                String::from("defaultplugin"), //TODO Remove defaultplugin
                request.get_ref().id.clone(),
                enabler,
            )
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;

        Ok(Response::new(Empty::default()))
    }

    async fn remove_enabler(
        &self,
        request: Request<EnablerQuery>,
    ) -> Result<Response<Empty>, Status> {
        self.field
            .control_system()
            .await
            .remove_enabler(String::from("defaultplugin"), request.get_ref().id.clone()) //TODO Remove defaultplugin
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;
        Ok(Response::new(Empty::default()))
    }

    async fn update_estopper(
        &self,
        request: Request<EstopperConfig>,
    ) -> Result<Response<Empty>, Status> {
        let estopper: estopper::Estopper = match &request.get_ref().estopper {
            Some(Estopper::AllEstopper(config)) => {
                estopper::AllEstopper::new(request.get_ref().name.clone(), config.active)
            }
            Some(Estopper::AllianceStationEstopper(config)) => {
                let mut estopped_stations: Vec<AllianceStation> = Vec::new();
                for station in config.estopped_stations.iter() {
                    estopped_stations.push(AllianceStation::from_byte(station.clone() as u8));
                }
                estopper::AllianceStationEstopper::new(
                    request.get_ref().name.clone(),
                    estopped_stations,
                )
            }
            Some(Estopper::TeamNumberEstopper(config)) => {
                let mut estopped_teams: Vec<u16> = Vec::new();
                for team in config.estopped_team_numbers.iter() {
                    estopped_teams.push(team.clone() as u16);
                }
                estopper::TeamNumberEstopper::new(request.get_ref().name.clone(), estopped_teams)
            }
            _ => return Err(Status::invalid_argument("Configuration message not proper")),
        };
        self.field
            .control_system()
            .await
            .update_estopper(
                String::from("defaultplugin"), //TODO Remove defaultplugin
                request.get_ref().id.clone(),
                estopper,
            )
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;

        Ok(Response::new(Empty::default()))
    }

    async fn remove_estopper(
        &self,
        request: Request<EstopperQuery>,
    ) -> Result<Response<Empty>, Status> {
        self.field
            .control_system()
            .await
            .remove_estopper(String::from("defaultplugin"), request.get_ref().id.clone()) //TODO Remove defaultplugin
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;
        Ok(Response::new(Empty::default()))
    }

    type OnDriverStationCreateStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_create(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationCreateStream>, Status> {
        let (tx, rx) = mpsc::channel::<Result<DriverStation, Status>>(1);
        let mut recv = self
            .field
            .driverstations()
            .await
            .create_driverstation_receiver()
            .await;

        tokio::spawn(async move {
            loop {
                let raw = recv.recv().await;
                if raw.is_err() {
                    break;
                }
                let res = tx.send(Ok(raw.unwrap())).await;
                if res.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type OnDriverStationDeleteStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_delete(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationDeleteStream>, Status> {
        let (tx, rx) = mpsc::channel::<Result<DriverStation, Status>>(1);
        let mut recv = self
            .field
            .driverstations()
            .await
            .delete_driverstation_receiver()
            .await;

        tokio::spawn(async move {
            loop {
                let raw = recv.recv().await;
                if raw.is_err() {
                    break;
                }
                let res = tx.send(Ok(raw.unwrap())).await;
                if res.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn get_driver_stations(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<DriverStations>, Status> {
        Ok(Response::new(
            self.field
                .driverstations()
                .await
                .get_driverstations_rpc()
                .await,
        ))
    }

    async fn get_driver_station(
        &self,
        request: Request<DriverStationQuery>,
    ) -> Result<Response<DriverStation>, Status> {
        if request.get_ref().query_type == DriverStationQueryType::Teamnumber as i32 {
            let ds = self
                .field
                .driverstations()
                .await
                .get_driverstation_by_team_number(request.get_ref().team_number as u16)
                .await
                .ok_or(Status::unavailable("Can't find driverstation"))?;
            Ok(Response::new(ds.to_rpc().await))
        } else {
            let ds = self
                .field
                .driverstations()
                .await
                .get_driverstation_by_position(enums::AllianceStation::from_byte(
                    request.get_ref().alliance_station as u8,
                ))
                .await
                .ok_or(Status::unavailable("Can't find driverstation"))?;
            Ok(Response::new(ds.to_rpc().await))
        }
    }

    async fn add_driver_station(
        &self,
        request: Request<DriverStationParams>,
    ) -> Result<Response<DriverStation>, Status> {
        let ds = driverstation::DriverStation::new(
            request.get_ref().team_number as u16,
            enums::AllianceStation::from_byte(request.get_ref().alliance_station as u8),
            Mode::Autonomous
        );
        self.field
            .driverstations()
            .await
            .add_driverstation(ds.clone())
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;
        Ok(Response::new(ds.to_rpc().await))
    }

    async fn delete_driver_station(
        &self,
        request: Request<DriverStationParams>,
    ) -> Result<Response<Empty>, Status> {
        self.field
            .driverstations()
            .await
            .delete_driverstation(request.get_ref().team_number as u16)
            .await
            .map_err(|e| Status::unavailable(e.to_string()))?;
        Ok(Response::new(Empty {}))
    }

    async fn update_driver_station_mode(
        &self,
        request: Request<DriverStationUpdateMode>,
    ) -> Result<Response<Empty>, Status> {
        self.field
            .driverstations()
            .await
            .get_driverstation_by_team_number(request.get_ref().team_number as u16)
            .await
            .ok_or(Status::unavailable("Can't find driverstation"))?
            .update_mode(Mode::from_byte(request.get_ref().mode as u8))
            .await;
        Ok(Response::new(Empty {}))
    }
}

pub struct NetworkConfiguratorApiImpl {
    pub field: Field,
}

#[tonic::async_trait]
impl NetworkConfiguratorApi for NetworkConfiguratorApiImpl {
    async fn get_driver_stations(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<DriverStations>, Status> {
        Ok(Response::new(
            self.field
                .driverstations()
                .await
                .get_driverstations_rpc()
                .await,
        ))
    }

    type OnDriverStationCreateStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_create(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationCreateStream>, Status> {
        let (tx, rx) = mpsc::channel::<Result<DriverStation, Status>>(1);
        let mut recv = self
            .field
            .driverstations()
            .await
            .create_driverstation_receiver()
            .await;

        tokio::spawn(async move {
            loop {
                let raw = recv.recv().await;
                if raw.is_err() {
                    break;
                }
                let res = tx.send(Ok(raw.unwrap())).await;
                if res.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    type OnDriverStationDeleteStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_delete(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationDeleteStream>, Status> {
        let (tx, rx) = mpsc::channel::<Result<DriverStation, Status>>(1);
        let mut recv = self
            .field
            .driverstations()
            .await
            .delete_driverstation_receiver()
            .await;

        tokio::spawn(async move {
            loop {
                let raw = recv.recv().await;
                if raw.is_err() {
                    break;
                }
                let res = tx.send(Ok(raw.unwrap())).await;
                if res.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn update_driver_station_expected_ip(
        &self,
        request: Request<DriverStationUpdateExpectedIp>,
    ) -> Result<Response<DriverStation>, Status> {
        if let Some(ds) = self
            .field
            .driverstations()
            .await
            .get_driverstation_by_team_number(request.get_ref().team_number as u16)
            .await
        {
            if let Ok(cidr) = request.get_ref().expected_ip.parse::<AnyIpCidr>() {
                ds.update_expected_ip(cidr).await;
                return Ok(Response::new(ds.to_rpc().await));
            } else {
                return Err(Status::invalid_argument("cidr not valid"));
            }
        } else {
            return Err(Status::invalid_argument(
                "team number is not a current driver station",
            ));
        }
    }
}
