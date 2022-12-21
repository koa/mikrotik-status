use std::backtrace::Backtrace;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;
use std::sync::Arc;

use thiserror::Error;

use crate::topology::query::NetboxError;

pub type Result<T> = std::result::Result<T, BackendError>;

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

#[derive(Error, Debug, Clone)]
pub enum BackendError {
    #[error("Error calling api: {error}")]
    Reqwest {
        error: Arc<reqwest::Error>,
        backtrace: Arc<Backtrace>,
    },
    #[error("Error from remote server: {error}")]
    Graphql {
        error: Arc<GraphqlError>,
        backtrace: Arc<Backtrace>,
    },
    #[error("Error Parsing integer: {error}")]
    ParseInt {
        error: ParseIntError,
        backtrace: Arc<Backtrace>,
    },
    #[error("Multiple Errors")]
    Umbrella(Vec<BackendError>),
    #[error("No ip address found")]
    MissingIpAddress(),
    #[error("Error from Netbox: {error}")]
    NetboxError {
        error: NetboxError,
        backtrace: Arc<Backtrace>,
    },
    #[error("Error loading config: {error}\n{backtrace}")]
    ConfigError {
        error: Arc<clap::Error>,
        backtrace: Arc<Backtrace>,
    },
}

impl From<&BackendError> for BackendError {
    fn from(value: &BackendError) -> Self {
        value.clone()
    }
}

impl From<clap::Error> for BackendError {
    fn from(error: clap::Error) -> Self {
        BackendError::ConfigError {
            error: Arc::new(error),
            backtrace: Arc::new(Backtrace::force_capture()),
        }
    }
}

impl From<NetboxError> for BackendError {
    fn from(error: NetboxError) -> Self {
        BackendError::NetboxError {
            error,
            backtrace: Arc::new(Backtrace::force_capture()),
        }
    }
}

impl From<ParseIntError> for BackendError {
    fn from(error: ParseIntError) -> Self {
        BackendError::ParseInt {
            error,
            backtrace: Arc::new(Backtrace::force_capture()),
        }
    }
}

impl From<reqwest::Error> for BackendError {
    fn from(error: reqwest::Error) -> Self {
        BackendError::Reqwest {
            error: Arc::new(error),
            backtrace: Arc::new(Backtrace::force_capture()),
        }
    }
}
