use async_graphql::{EmptySubscription, ObjectType, Schema, SubscriptionType};
use async_graphql_poem::GraphQL;

use crate::{
    field::Field,
    graph::{mutation::Mutation, query::Query},
};


pub fn create_schema(field: Field) -> Schema<Query, Mutation, EmptySubscription> {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(field)
        .finish()
}

pub fn create_graphql_endpoint<Q, M, S>(schema: Schema<Q, M, S>) -> GraphQL<Schema<Q, M, S>> {
    GraphQL::new(schema)
}

pub struct SdlEndpoint<Q, M, S>(Schema<Q, M, S>);

impl<Q, M, S> poem::Endpoint for SdlEndpoint<Q, M, S>
where
    Q: ObjectType + 'static,
    M: ObjectType + 'static,
    S: SubscriptionType + 'static,
{
    type Output = poem::Response;

    async fn call(&self, _req: poem::Request) -> poem::Result<poem::Response> {
        let sdl = self.0.sdl();
        Ok(poem::Response::builder()
            .status(poem::http::StatusCode::OK)
            .body(sdl))
    }
}

pub fn create_sdl_endpoint<Q, M, S>(schema: Schema<Q, M, S>) -> SdlEndpoint<Q, M, S> {
    SdlEndpoint(schema)
}
