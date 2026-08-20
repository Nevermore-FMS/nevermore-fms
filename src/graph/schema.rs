use async_graphql::{EmptySubscription, Schema};
use async_graphql_poem::{GraphQLRequest, GraphQLResponse};
use poem::web::headers::{Authorization, HeaderMapExt, authorization::Bearer};

use crate::{
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
) -> GraphQLResponse {
    let mut gql_req = gql_req.0;

    gql_req = gql_req.data(fms_core.0.clone());

    if let Some(auth_header) = poem_req.headers().typed_get::<Authorization<Bearer>>() {
        // TODO Move this auth to its own module that handles the hashing and token type
        if let Ok(Some(auth_token_details)) = fms_core
            .main_db()
            .get_authentication_token_by_token_hash(auth_header.token().to_owned())
            && auth_token_details.target_type == "user"
            && let Ok(Some(user)) = fms_core
                .main_db()
                .get_user_by_id(auth_token_details.target_id)
        {
            gql_req = gql_req.data(user);
        }
    }

    schema.execute(gql_req).await.into()
}

#[allow(clippy::needless_pass_by_value)]
#[poem::handler]
pub fn sdl_endpoint(schema: poem::web::Data<&GQLSchema>) -> poem::Response {
    let sdl = schema.sdl();
    poem::Response::builder()
        .status(poem::http::StatusCode::OK)
        .body(sdl)
}
