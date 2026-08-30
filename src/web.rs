pub mod openid;
pub mod graph;

use std::net::SocketAddr;

use log::info;
use poem::{
    EndpointExt, Route, Server, get, http::Method, listener::TcpListener, middleware::Cors, post,
};
use tokio_util::sync::CancellationToken;

use crate::{fmscore::FMSCore};

pub async fn run(
    web_address: SocketAddr,
    fms_core: FMSCore,
    cancellation_token: CancellationToken,
) -> anyhow::Result<()> {
    let gql_schema = graph::schema::create_schema();
    let app = Route::new()
        .at("/api/graphql", post(graph::schema::graphql_endpoint))
        .at("/api/schema.graphql", get(graph::schema::sdl_endpoint))

        .at("/openid/.well-known/openid-configuration", get(openid::openid_configuration_endpoint))
        .at("/openid/keys", get(openid::openid_jwks_endpoint))
        .at("/openid/authorize", get(openid::openid_authorization_endpoint))
        
        .with(
            Cors::new()
                .allow_method(Method::GET)
                .allow_method(Method::POST),
        )
        .data(gql_schema)
        .data(fms_core);

    info!("Web server started on {web_address}");

    let server = Server::new(TcpListener::bind(web_address));

    let join_handle = tokio::task::Builder::new()
        .name("Web Server")
        .spawn(async move {
            server
                .run_with_graceful_shutdown(app, cancellation_token.cancelled(), None)
                .await
        })?;

    join_handle
        .await
        .map_err(anyhow::Error::from)?
        .map_err(anyhow::Error::from)
}
