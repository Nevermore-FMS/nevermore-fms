use std::net::SocketAddr;

use log::info;
use poem::{
    EndpointExt, Route, Server, get, http::Method, listener::TcpListener, middleware::Cors, post,
};
use tokio::task::JoinHandle;

use crate::{field::Field, graph};

pub fn start_server(
    web_address: SocketAddr,
    field: Field,
) -> anyhow::Result<JoinHandle<Result<(), std::io::Error>>> {
    let schema = graph::schema::create_schema(field);
    let app = Route::new()
        .at(
            "/api/graphql",
            post(graph::schema::create_graphql_endpoint(schema.clone())),
        )
        .at(
            "/api/schema.graphql",
            get(graph::schema::create_sdl_endpoint(schema)),
        )
        .with(
            Cors::new()
                .allow_method(Method::GET)
                .allow_method(Method::POST),
        );

    info!("Web server started on {}", web_address);

    let server = Server::new(TcpListener::bind(web_address));

    let join_handle = tokio::task::Builder::new()
        .name("Web Server")
        .spawn(server.run(app))?;

    Ok(join_handle)
}
