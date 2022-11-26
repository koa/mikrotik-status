use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "./graphql/schema.graphql",
    query_path = "./src/graphql/settings.graphql",
    response_derives = "Debug"
)]
pub struct Settings;
