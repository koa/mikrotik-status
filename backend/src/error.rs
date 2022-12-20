use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

use thiserror::Error;

use crate::topology::query::NetboxError;

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
    #[error("Error calling api: {error}")]
    Reqwest {
        error: reqwest::Error,
        backtrace: Box<Backtrace>,
    },
    #[error("Error from remote server: {error}")]
    Graphql {
        error: GraphqlError,
        backtrace: Box<Backtrace>,
    },
    #[error("Error Parsing integer: {error}")]
    ParseInt {
        error: ParseIntError,
        backtrace: Box<Backtrace>,
    },
    #[error("Multiple Errors")]
    Umbrella(Vec<BackendError>),
    #[error("No ip address found")]
    MissingIpAddress(),
    #[error("Error from Netbox: {error}")]
    NetboxError {
        error: NetboxError,
        backtrace: Box<Backtrace>,
    },
}

impl From<NetboxError> for BackendError {
    fn from(error: NetboxError) -> Self {
        BackendError::NetboxError {
            error,
            backtrace: Box::new(Backtrace::force_capture()),
        }
    }
}

impl From<ParseIntError> for BackendError {
    fn from(error: ParseIntError) -> Self {
        BackendError::ParseInt {
            error,
            backtrace: Box::new(Backtrace::force_capture()),
        }
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(error: reqwest::Error) -> Self {
        BackendError::Reqwest {
            error,
            backtrace: Box::new(Backtrace::force_capture()),
        }
    }
}
