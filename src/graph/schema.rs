use async_graphql::*;

pub type NevermoreSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

pub fn create_schema() -> NevermoreSchema {
    Schema::new(Query, EmptyMutation, EmptySubscription)
}