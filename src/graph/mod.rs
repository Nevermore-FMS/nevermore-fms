use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_poem::GraphQL;

use crate::{field::Field, graph::query::Query};

pub mod query;
mod types;
mod inputs;

pub fn provide_graphql(field: Field) -> GraphQL<Schema<Query, EmptyMutation, EmptySubscription>> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(field)
        .finish();
    GraphQL::new(schema)
}
