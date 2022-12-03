use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema/netbox.graphql",
    query_path = "src/topology/list_devices.graphql",
    response_derives = "Debug",
    variables_derives = "Default,Debug"
)]
pub struct ListAllDevices;
