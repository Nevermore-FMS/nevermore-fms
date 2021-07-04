use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_warp::{graphql_subscription, Response};
use std::convert::Infallible;
use warp::{http::Response as HttpResponse, Filter};

use crate::application::ThreadSafeApplication;

pub mod schema;

pub async fn start(application: ThreadSafeApplication) {
    let schema = schema::create_schema(application);
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

    warp::serve(api_routes).run(([0, 0, 0, 0], 8000)).await;
}
