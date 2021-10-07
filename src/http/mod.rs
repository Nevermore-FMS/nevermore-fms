use async_graphql_warp::{graphql_subscription, Response};
use std::{convert::Infallible, net::SocketAddr};
use warp::{
    Filter, 
    http::Method
};
 

use crate::{application::ThreadSafeApplication, http::graph::NevermoreSchema};

pub mod graph;
pub mod resources;

pub struct AuthorizationHeader(String);

impl AuthorizationHeader {
    pub fn as_bearer(&self) -> anyhow::Result<String> {
        let split = &self.0.split(" ").collect::<Vec<&str>>();

        if *split
            .get(0)
            .ok_or(anyhow::anyhow!("first part of auth header not found"))?
            == "Bearer"
        {
            let mut maybe_token = split.get(1);
            if let Some(token) = maybe_token.take() {
                Ok(token.to_string())
            } else {
                Err(anyhow::anyhow!("second part of auth header not found"))
            }
        } else {
            Err(anyhow::anyhow!("not a bearer token"))
        }
    }
}

pub async fn start(application: ThreadSafeApplication, http_addr: SocketAddr) {
    let cloned_application = application.clone();
    let application_filter = warp::any().map(move || cloned_application.clone());

    let schema = graph::create_schema(application.clone());
    let graphql_post = warp::header::optional::<String>("authorization")
        .and(application_filter.clone())
        .and(async_graphql_warp::graphql(schema.clone()))
        .and_then(
            |token,
             application: ThreadSafeApplication,
             (schema, mut request): (NevermoreSchema, async_graphql::Request)| async move {
                if let Some(token) = token {
                    let token = AuthorizationHeader(token).as_bearer();
                    if token.is_ok() {
                        let token = token.unwrap();
                        let application = application.clone();
                        let application = application.read().await;

                        let maybe_session = application
                            .session_storage
                            .write()
                            .await
                            .verify_token(application.database.clone(), token.clone())
                            .await;

                        if maybe_session.is_ok() {
                            let session = maybe_session.unwrap();

                            request = request.data(session);
                            request = request.data(token);
                        }
                    }
                }

                Ok::<_, Infallible>(Response::from(schema.execute(request).await))
            },
        );

    #[cfg(feature = "developer")]
    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
        use warp::http::Response as HttpResponse;

        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
            ))
    });

    let graphql_routes = graphql_subscription(schema);

    #[cfg(feature = "developer")]
    let graphql_routes = graphql_routes.or(graphql_playground);

    let graphql_routes = graphql_routes.or(graphql_post);

    let routes = warp::path!("graphql").and(graphql_routes);

    let index_html = warp::path!("devtools").and_then(resources::serve_index_devtools);
    let dist = warp::path("devtools")
        .and(warp::path::tail())
        .and_then(resources::serve_devtools);

    let routes = routes.or(index_html.or(dist));

    #[cfg(feature = "developer")]
    let routes = routes.or(warp::path!("inspector")
        .and(warp::ws())
        .and(application_filter)
        .map(|ws: warp::ws::Ws, application| {
            ws.on_upgrade(|websocket| inspector_connected(websocket, application))
        }));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET, Method::POST, Method::DELETE])
        .allow_headers(vec!["authorization", "content-type"]);
    warp::serve(routes.with(cors)).run(http_addr).await;
}

#[cfg(feature = "developer")]
use warp::ws::{Message, WebSocket};

#[cfg(feature = "developer")]
async fn inspector_connected(ws: WebSocket, application: ThreadSafeApplication) {
    use deno_core::{error::AnyError, InspectorSessionProxy};
    use futures::{StreamExt, TryFutureExt, TryStreamExt};

    // The 'outbound' channel carries messages sent to the websocket.
    let (outbound_tx, outbound_rx) = futures::channel::mpsc::unbounded();

    // The 'inbound' channel carries messages received from the websocket.
    let (inbound_tx, inbound_rx) = futures::channel::mpsc::unbounded();

    let proxy = InspectorSessionProxy {
        tx: outbound_tx,
        rx: inbound_rx,
    };

    let locked_application = application.read().await;

    let pump = async move {
        let (websocket_tx, websocket_rx) = ws.split();

        let outbound_pump = outbound_rx
            .map(|(_maybe_call_id, msg)| Message::text(msg))
            .map(Ok)
            .forward(websocket_tx)
            .map_err(|_| ());

        let inbound_pump = websocket_rx
            .map(|result| {
                let result = result.map(|msg| msg.into_bytes()).map_err(AnyError::from).unwrap();
                inbound_tx.unbounded_send(result)
            })
            .map_err(|_| ())
            .try_collect::<()>();

        let _ = futures::future::try_join(outbound_pump, inbound_pump).await;
    };

    let mut inspector_sender = locked_application.inspector_sender.clone();
    let mut closing_sender = locked_application.closing_sender.clone();

    // Unlock the Mutex.
    drop(locked_application);

    if let Some(inspector_sender) = inspector_sender.take() {
        let _ = inspector_sender.unbounded_send(proxy);
        if let Some(sender) = closing_sender.take() {
            let mut receiver = sender.subscribe();
            tokio::select! {
                _ = pump => {}
                _ = receiver.recv() => {}
            }
        }
    }
}
