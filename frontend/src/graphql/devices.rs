use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "src/graphql/list_devices.graphql",
    response_derives = "Debug,Eq,PartialEq,Clone"
)]
pub struct ListDevices;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "src/graphql/ping_device.graphql",
    response_derives = "Debug,Eq,PartialEq,Clone"
)]
pub struct PingDevice;
