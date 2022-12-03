use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug)]
pub struct GraphqlError(Vec<graphql_client::Error>);

impl GraphqlError {
    pub fn new(errors: Option<Vec<graphql_client::Error>>) -> GraphqlError {
        GraphqlError(errors.unwrap_or(Vec::new()))
    }
}

impl Display for GraphqlError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Error for GraphqlError {}

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Error calling api")]
    Reqwest(#[from] reqwest::Error),
    #[error("Error from remote server")]
    Graphql(GraphqlError),
    #[error("Error Parsing integer")]
    ParseInt(#[from] ParseIntError),
    #[error("Multiple Errors")]
    Umbrella(Vec<BackendError>),
    #[error("No ip address found")]
    MissingIpAddress(),
    #[error("Device {0} not found")]
    DeviceNotFound(u32),
}
