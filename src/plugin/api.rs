use log::info;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use crate::field::Field;

use super::PluginManager;
use super::rpc::fms_server::{Fms, FmsServer};

use super::rpc::{Empty, PluginInfo, FieldState, DriverStations, DriverStationQuery, DriverStation, DriverStationParams};

pub struct FmsImpl {
    pub plugin_manager: PluginManager,
    pub field: Field
}

#[tonic::async_trait]
impl Fms for FmsImpl {
    async fn register_plugin(&self, request: tonic::Request<PluginInfo>) -> Result<Response<Empty>, Status> {
        info!("Register Plugin: {}, {}, {:?}", request.get_ref().name, request.get_ref().version, request.get_ref().setup_url);
        Ok(Response::new(Empty{}))
    }

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
                    break
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

    async fn get_field_state(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<FieldState>, Status> {
        Err(Status::unknown("TODO"))
    }
    
    async fn set_field_state(
        &self,
        request: Request<FieldState>,
    ) -> Result<Response<FieldState>, Status> {
        Err(Status::unknown("TODO"))
    }

    type OnDriverStationUpdateStream = ReceiverStream<Result<DriverStation, Status>>;

    async fn on_driver_station_update(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::OnDriverStationUpdateStream>, Status> {
        Err(Status::unknown("TODO"))
    }

    async fn get_driver_stations(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<DriverStations>, Status> {
        Err(Status::unknown("TODO"))
    }

    async fn get_driver_station(
        &self,
        request: Request<DriverStationQuery>,
    ) -> Result<Response<DriverStation>, Status> {
        Err(Status::unknown("TODO"))
    }

    async fn set_driver_station(
        &self,
        request: Request<DriverStationParams>,
    ) -> Result<Response<DriverStation>, Status> {
        //self.field.driverstations().await.add_driverstation(driverstation)
        // TODO: Chase: Marking where I left off
        Err(Status::unknown("TODO"))
    }

    async fn delete_driver_station(
        &self,
        request: Request<DriverStationParams>,
    ) -> Result<Response<Empty>, Status> {
        Err(Status::unknown("TODO"))
    }

}