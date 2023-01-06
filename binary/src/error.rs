use std::backtrace::Backtrace;

use actix_4_jwt_auth::OIDCValidationError;
use thiserror::Error;

use backend::error::BackendError;

pub type Result<T> = std::result::Result<T, BinaryError>;

#[derive(Error, Debug)]
pub enum BinaryError {
    #[error("An IO Error happened: {error}\n{backtrace}")]
    IO {
        error: std::io::Error,
        backtrace: Box<Backtrace>,
    },
    #[error("An Error from prometheus: {error}\n{backtrace}")]
    Prometheus {
        error: prometheus::Error,
        backtrace: Box<Backtrace>,
    },
    #[error("An Error from Backend: {error}\n{backtrace}")]
    Backend {
        error: BackendError,
        backtrace: Box<Backtrace>,
    },
    #[error("Cannot init OIDC validation from {issuer}: {error}\n{backtrace}")]
    OIDCValidation {
        issuer: String,
        error: OIDCValidationError,
        backtrace: Box<Backtrace>,
    },
}

impl From<std::io::Error> for BinaryError {
    fn from(error: std::io::Error) -> Self {
        BinaryError::IO {
            error,
            backtrace: Box::new(Backtrace::capture()),
        }
    }
}

impl From<prometheus::Error> for BinaryError {
    fn from(error: prometheus::Error) -> Self {
        BinaryError::Prometheus {
            error,
            backtrace: Box::new(Backtrace::capture()),
        }
    }
}

impl From<BackendError> for BinaryError {
    fn from(error: BackendError) -> Self {
        BinaryError::Backend {
            error,
            backtrace: Box::new(Backtrace::capture()),
        }
    }
}

impl BinaryError {
    pub fn oidc_validation_error(issuer: String, error: OIDCValidationError) -> Self {
        BinaryError::OIDCValidation {
            issuer,
            error,
            backtrace: Box::new(Backtrace::capture()),
        }
    }
}
