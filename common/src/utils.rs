use serde::{Deserialize, Serialize};

pub const REST_ENDPOINT: &str = if cfg!(debug_assertions) {
    "http://localhost:8000"
} else {
    //"https://inthetrenches.cloud:443"
    "http://localhost:8000"
};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome<T> {
    Success(T),
    Failure(String),
}
