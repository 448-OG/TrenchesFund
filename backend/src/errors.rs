use serde::Deserialize;

pub type BackendResult<T> = Result<T, BackendError>;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum JsonResponse<T, U> {
    Success(T),
    Failure(U),
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum BackendError {
    #[error("KV: {0}")]
    Kv(String),
    #[error("The KV is yet to be initialized")]
    KvUninitialized,
    #[error("The KV is already initialized")]
    KvAlreadyInitialized,
    #[error("They key already exists in the KV")]
    KvAlreadyExists,
    #[error("The key was not found in the KV")]
    KvKeyNotFound,
    #[error("Permission denied on the resource requested")]
    PermissionDenied,
    #[error("Unable to deserialize JSON")]
    JsonDeserialize,
    #[error("Unable to deserialize or deserialize bytes")]
    BincodeError,
}

impl From<surrealkv::Error> for BackendError {
    fn from(value: surrealkv::Error) -> Self {
        Self::Kv(value.to_string())
    }
}

impl From<serde_json::Error> for BackendError {
    fn from(_: serde_json::Error) -> Self {
        Self::JsonDeserialize
    }
}

impl From<bincode::Error> for BackendError {
    fn from(_: bincode::Error) -> Self {
        Self::BincodeError
    }
}
