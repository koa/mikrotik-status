use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use crate::api::query::Query;

pub mod device;
pub mod location;
pub mod query;
pub mod settings;
pub mod site;

pub type GraphqlSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> GraphqlSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription).finish()
}
