use graphql_client::GraphQLQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "src/graphql/list_sites.graphql",
    response_derives = "Debug,Eq,PartialEq,Clone"
)]
pub struct ListSites;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "graphql/schema.graphql",
    query_path = "src/graphql/get_site_details.graphql",
    response_derives = "Debug,Eq,PartialEq,Clone"
)]
pub struct GetSiteDetails;
