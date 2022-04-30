use crate::field::{Field, driverstation, enums};
use log::info;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

use super::rpc::generic_api_server::{GenericApi, GenericApiServer};
use super::PluginManager;

use super::rpc::{
    DriverStation, DriverStationParams, DriverStationQuery, DriverStations, Empty, FieldState, DriverStationQueryType,
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
        let event_name = self.field.event_name().await;
        let tournament_level = self.field.tournament_level().await.to_byte() as i32;
        let match_number = self.field.match_number().await as u32;
        let play_number = self.field.play_number().await as u32;
        let time_left = self.field.time_remaining().await as f32;

        Ok(Response::new(FieldState {
            event_name,
            tournament_level,
            match_number,
            play_number,
            time_left
        }))
    }

    async fn set_field_state(
        &self,
        request: Request<FieldState>,
    ) -> Result<Response<FieldState>, Status> {
        self.field.update_rpc(request.get_ref().clone()).await;
        Ok(Response::new(request.get_ref().clone()))
    }

    type OnDriverStationCreateStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_create(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationCreateStream>, Status> {
        Err(Status::unknown("TODO"))
    }

    type OnDriverStationUpdateStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_update(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationUpdateStream>, Status> {
        Err(Status::unknown("TODO"))
    }

    type OnDriverStationDeleteStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_delete(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationDeleteStream>, Status> {
        Err(Status::unknown("TODO"))
    }

    async fn get_driver_stations(
        &self,
        _: Request<Empty>,
    ) -> Result<Response<DriverStations>, Status> {
        Ok(Response::new(self.field.driverstations().await.get_driverstations_rpc().await))
    }

    async fn get_driver_station(
        &self,
        request: Request<DriverStationQuery>,
    ) -> Result<Response<DriverStation>, Status> {
        if request.get_ref().query_type == DriverStationQueryType::Teamnumber as i32 {
            let ds = self.field.driverstations().await.get_driverstation_by_team_number(request.get_ref().team_number as u16).await.ok_or(Status::unavailable("Can't find driverstation"))?;
            Ok(Response::new(ds.to_rpc().await))
        } else {
            let ds = self.field.driverstations().await.get_driverstation_by_position(enums::AllianceStation::from_byte(request.get_ref().alliance_station as u8)).await.ok_or(Status::unavailable("Can't find driverstation"))?;
            Ok(Response::new(ds.to_rpc().await))
        }
    }

    async fn add_driver_station(
        &self,
        request: Request<DriverStationParams>,
    ) -> Result<Response<DriverStation>, Status> {
        let ds = driverstation::DriverStation::new(request.get_ref().team_number as u16, enums::AllianceStation::from_byte(request.get_ref().alliance_station as u8), );
        self.field.driverstations().await.add_driverstation(ds.clone()).await.map_err(|_| Status::unavailable("cannot add driverstation"))?;
        Ok(Response::new(ds.to_rpc().await))
    }

    async fn update_driver_station(
        &self,
        request: Request<DriverStationParams>,
    ) -> Result<Response<DriverStation>, Status> {
        let ds = self.field.driverstations().await.get_driverstation_by_team_number(request.get_ref().team_number as u16).await.ok_or(Status::unavailable("Can't find driverstation"))?;
        ds.update(request.get_ref().clone()).await;
        Ok(Response::new(ds.to_rpc().await))
    }

    async fn delete_driver_station(
        &self,
        request: Request<DriverStationParams>,
    ) -> Result<Response<Empty>, Status> {
        self.field.driverstations().await.delete_driverstation(request.get_ref().team_number as u16).await.map_err(|_| Status::unavailable("cannot delete driverstation"))?;
        Ok(Response::new(Empty{}))
    }
}
