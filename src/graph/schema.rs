use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use log::error;
use poem::{
    http::StatusCode,
    web::headers::{Authorization, HeaderMapExt, authorization::Bearer},
};

use crate::{
    authentication::exchange_token,
    fmscore::FMSCore,
    graph::{mutation::Mutation, query::Query},
};

type GQLSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema() -> GQLSchema {
    Schema::build(Query, Mutation, EmptySubscription).finish()
}

#[poem::handler]
pub async fn graphql_endpoint(
    schema: poem::web::Data<&GQLSchema>,
    fms_core: poem::web::Data<&FMSCore>,
    poem_req: &poem::Request,
    gql_req: GraphQLRequest,
) -> poem::Result<GraphQLResponse> {
    let mut gql_req = gql_req.0;

    gql_req = gql_req.data(fms_core.0.clone());

    if let Some(auth_header) = poem_req.headers().typed_get::<Authorization<Bearer>>() {
        let auth_ctx = match exchange_token(&fms_core.main_db(), auth_header.token()) {
            Ok(ctx) => ctx,
            Err(e) => {
                error!("Error when fulfilling graphql request: {e}");
                return Err(poem::Error::from_status(StatusCode::INTERNAL_SERVER_ERROR));
            }
        };

        if let Some(auth_ctx) = auth_ctx {
            gql_req = gql_req.data(auth_ctx);
        }
    }

    Ok(schema.execute(gql_req).await.into())
}

#[allow(clippy::needless_pass_by_value)]
#[poem::handler]
pub fn sdl_endpoint(schema: poem::web::Data<&GQLSchema>) -> poem::Response {
    let sdl = schema.sdl();
    poem::Response::builder()
        .status(poem::http::StatusCode::OK)
        .body(sdl)
}
