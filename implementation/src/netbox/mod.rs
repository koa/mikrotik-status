use std::fmt::Debug;

use graphql_client::{GraphQLQuery, Response};
use log::debug;
use reqwest::header::AUTHORIZATION;

use crate::config::CONFIG;
use crate::error::{BackendError, GraphqlError};

pub mod graphql;

pub async fn query<Q>(request: Q::Variables) -> Result<Q::ResponseData, BackendError>
where
    Q: GraphQLQuery,
    Q::Variables: Debug,
    Q::ResponseData: Debug,
{
    let request_body = Q::build_query(request);
    let name = request_body.operation_name;
    debug!("Graphql Request {name}: {request_body:?}");
    let client = reqwest::Client::new();
    let response: Response<Q::ResponseData> = client
        .post(&CONFIG.netbox.endpoint)
        .json(&request_body)
        .header(AUTHORIZATION, format!("Token {}", &CONFIG.netbox.token))
        .send()
        .await?
        .json()
        .await?;
    if let Some(data) = response.data {
        debug!("Graphql Response {name}: {data:?}");
        Ok(data)
    } else {
        let error = GraphqlError::new(response.errors);
        debug!("Graphql Error {name}: {error:?}");
        Err(BackendError::Graphql(error))
    }
}
