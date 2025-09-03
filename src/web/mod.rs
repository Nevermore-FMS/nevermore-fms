use std::net::SocketAddr;

use log::info;
use poem::{
    EndpointExt, Route, Server, http::Method, listener::TcpListener, middleware::Cors, post,
};

use crate::{field::Field, graph};

pub async fn start_server(web_address: SocketAddr, field: Field) {
    let app = Route::new()
        .at("/api/graphql", post(graph::provide_graphql(field)))
        .with(
            Cors::new()
                .allow_method(Method::GET)
                .allow_method(Method::POST),
        );

    info!("Web server started on {}", web_address);
    Server::new(TcpListener::bind(web_address))
        .run(app)
        .await
        .unwrap()
}
