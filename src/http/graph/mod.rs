use async_graphql::*;
use async_graphql::extensions::ApolloTracing;

use crate::application::ThreadSafeApplication;

pub mod node;
pub mod plugin;
pub mod user;
pub mod dev;
pub mod config;
pub mod guards;
pub mod field;
pub mod network;

pub type NevermoreSchema = Schema<Query, Mutation, Subscription>;

// Merged Queries

#[derive(MergedObject, Default)]
pub struct Query(node::NodeQuery, plugin::PluginQuery, user::UserQuery, dev::DevQuery, config::ConfigQuery, field::FieldQuery, network::NetworkQuery);

// Merged Mutations

#[derive(MergedObject, Default)]
pub struct Mutation(dev::DevMutation, config::ConfigMutation, field::FieldMutation, network::NetworkMutation);

// Merged Subscriptions

#[derive(MergedSubscription, Default)]
pub struct Subscription(dev::DevSubscription, field::FieldSubscription);

pub fn create_schema(application: ThreadSafeApplication) -> NevermoreSchema {
    Schema::build(Query::default(), Mutation::default(), Subscription::default())
        .data(application)
        .extension(ApolloTracing)
        .finish()
}
