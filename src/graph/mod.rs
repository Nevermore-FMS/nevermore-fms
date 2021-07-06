use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{graphql_subscription, Response};
use deno_core::{error::AnyError, InspectorSessionProxy};
use futures::{SinkExt, StreamExt, TryFutureExt, TryStreamExt};
use std::convert::Infallible;
use warp::{
    http::Response as HttpResponse,
    ws::{Message, WebSocket},
    Filter,
};
use log::info;

use crate::application::ThreadSafeApplication;

pub mod schema;

pub async fn start(application: ThreadSafeApplication) {
    let schema = schema::create_schema(application.clone());
    let graphql_post = async_graphql_warp::graphql(schema.clone()).and_then(
        |(schema, request): (schema::NevermoreSchema, async_graphql::Request)| async move {
            Ok::<_, Infallible>(Response::from(schema.execute(request).await))
        },
    );

    let graphql_playground = warp::path::end().and(warp::get()).map(|| {
        HttpResponse::builder()
            .header("content-type", "text/html")
            .body(playground_source(
                GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
            ))
    });

    let graphql_routes = graphql_subscription(schema)
        .or(graphql_playground)
        .or(graphql_post);

    let api_routes = warp::path!("graphql").and(graphql_routes);

    let application_filter = warp::any().map(move || application.clone());

    //#[cfg(feature = "developer")]
    let api_routes = api_routes.or(warp::path!("inspector")
        .and(warp::ws())
        .and(application_filter)
        .map(|ws: warp::ws::Ws, application| {
            ws.on_upgrade(|websocket| inspector_connected(websocket, application))
        }));

    warp::serve(api_routes).run(([0, 0, 0, 0], 8000)).await;
}

async fn inspector_connected(ws: WebSocket, application: ThreadSafeApplication) {
    // The 'outbound' channel carries messages sent to the websocket.
    info!("Connected");
    let (outbound_tx, outbound_rx) = futures::channel::mpsc::unbounded();

    // The 'inbound' channel carries messages received from the websocket.
    let (inbound_tx, inbound_rx) = futures::channel::mpsc::unbounded();

    let proxy = InspectorSessionProxy {
        tx: outbound_tx,
        rx: inbound_rx,
    };

    let pump = async move {
        let (websocket_tx, websocket_rx) = ws.split();

        let outbound_pump = outbound_rx
            .map(|(_maybe_call_id, msg)| Message::text(msg))
            .map(Ok)
            .forward(websocket_tx)
            .map_err(|_| ());

        let inbound_pump = websocket_rx
            .map(|result| {
                let result = result.map(|msg| msg.into_bytes()).map_err(AnyError::from);
                inbound_tx.unbounded_send(result)
            })
            .map_err(|_| ())
            .try_collect::<()>();

        let _ = futures::future::try_join(outbound_pump, inbound_pump).await;
    };

    let mut inspector_sender = application.lock().await.inspector_sender.clone();
    if let Some(inspector_sender) = inspector_sender.take() {
        let _ = inspector_sender.unbounded_send(proxy);
        pump.await;
    }
}
