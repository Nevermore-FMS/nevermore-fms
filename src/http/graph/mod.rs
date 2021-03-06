use async_graphql::extensions::ApolloTracing;
use async_graphql::*;

use crate::application::ThreadSafeApplication;

pub mod config;
pub mod dev;
pub mod field;
pub mod guards;
pub mod network;
pub mod node;
pub mod plugin;
pub mod user;
pub mod pub_sub;

pub type NevermoreSchema = Schema<Query, Mutation, Subscription>;

// Merged Queries

#[derive(MergedObject, Default)]
pub struct Query(
    node::NodeQuery,
    plugin::PluginQuery,
    user::UserQuery,
    dev::DevQuery,
    config::ConfigQuery,
    field::FieldQuery,
    network::NetworkQuery,
);

// Merged Mutations

#[derive(MergedObject, Default)]
pub struct Mutation(
    dev::DevMutation,
    config::ConfigMutation,
    field::FieldMutation,
    network::NetworkMutation,
    user::UserMutation,
    plugin::PluginMutation,
    pub_sub::PubSubMutation
);

// Merged Subscriptions

#[derive(MergedSubscription, Default)]
pub struct Subscription(dev::DevSubscription, field::FieldSubscription, pub_sub::PubSubSubscription);

pub fn create_schema(application: ThreadSafeApplication) -> NevermoreSchema {
    Schema::build(
        Query::default(),
        Mutation::default(),
        Subscription::default(),
    )
    .data(application)
    .extension(ApolloTracing)
    .finish()
}
