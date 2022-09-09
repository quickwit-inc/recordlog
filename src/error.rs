use std::io;

use thiserror::Error;

#[derive(Debug, Copy, Clone)]
pub struct AlreadyExists;

#[derive(Error, Debug)]
pub enum CreateQueueError {
    #[error("Already exists")]
    AlreadyExists,
    #[error("Io error: {0}")]
    IoError(#[from] io::Error),
}

impl From<AlreadyExists> for CreateQueueError {
    fn from(_: AlreadyExists) -> Self {
        CreateQueueError::AlreadyExists
    }
}

#[derive(Debug)]
pub struct TouchError;

#[derive(Error, Debug)]
pub enum TruncateError {
    #[error("Missing queue: {0}")]
    MissingQueue(String),
    #[error("Io error: {0}")]
    IoError(#[from] io::Error),
}

impl From<MissingQueue> for TruncateError {
    fn from(missing_queue: MissingQueue) -> Self {
        TruncateError::MissingQueue(missing_queue.0)
    }
}

#[derive(Error, Debug)]
pub enum AppendError {
    #[error("Io error: {0}")]
    IoError(#[from] io::Error),
    #[error("Missing queue: {0}")]
    MissingQueue(String),
    #[error("Past")]
    Past,
    #[error("Future")]
    Future,
}

impl From<MissingQueue> for AppendError {
    fn from(missing_queue: MissingQueue) -> Self {
        AppendError::MissingQueue(missing_queue.0)
    }
}

#[derive(Debug)]
pub struct MissingQueue(pub String);
