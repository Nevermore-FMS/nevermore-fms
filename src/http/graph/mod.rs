use async_graphql::*;

use crate::application::ThreadSafeApplication;

pub mod node;
pub mod plugin;
pub mod user;
pub mod dev;

pub type NevermoreSchema = Schema<Query, Mutation, Subscription>;

// Merged Queries

#[derive(MergedObject, Default)]
pub struct Query(node::NodeQuery, plugin::PluginQuery, user::UserQuery, dev::DevQuery);

// Merged Mutations

#[derive(MergedObject, Default)]
pub struct Mutation(dev::DevMutation);

// Merged Subscriptions

#[derive(MergedSubscription, Default)]
pub struct Subscription(dev::DevSubscription);

pub fn create_schema(application: ThreadSafeApplication) -> NevermoreSchema {
    Schema::build(Query::default(), Mutation::default(), Subscription::default())
        .data(application)
        .finish()
}
