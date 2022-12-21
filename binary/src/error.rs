use thiserror::Error;

use backend::error;

#[derive(Error, Debug)]
pub enum BinaryError {
    #[error("An IO Error happened: {0}")]
    IO(#[from] std::io::Error),
    #[error("An Error from prometheus: {0}")]
    Prometheus(#[from] prometheus::Error),
    #[error("An Error from Backend: {0}")]
    Backend(#[from] error::BackendError),
}
