use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/netbox/graphql/schema.graphql",
    query_path = "src/netbox/graphql/list_devices.graphql",
    response_derives = "Debug",
    variables_derives = "Default,Debug"
)]
pub struct ListDevices;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/netbox/graphql/schema.graphql",
    query_path = "src/netbox/graphql/get_device.graphql",
    response_derives = "Debug",
    variables_derives = "Debug"
)]
pub struct GetDevice;
