use std::net::SocketAddr;

use log::info;
use poem::{
    EndpointExt, Route, Server, get, http::Method, listener::TcpListener, middleware::Cors, post,
};

use crate::{
    field::Field,
    graph::{self, create_sdl_endpoint},
};

pub async fn start_server(web_address: SocketAddr, field: Field) -> anyhow::Result<()> {
    let schema = graph::create_schema(field);
    let app = Route::new()
        .at("/api/graphql", post(graph::create_graphql_endpoint(schema.clone())))
        .at("/api/schema.graphql", get(create_sdl_endpoint(schema)))
        .with(
            Cors::new()
                .allow_method(Method::GET)
                .allow_method(Method::POST),
        );

    info!("Web server started on {}", web_address);

    Server::new(TcpListener::bind(web_address))
        .run(app)
        .await?;

    Ok(())
}
