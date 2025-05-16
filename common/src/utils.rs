use serde::{Deserialize, Serialize};

pub const REST_ENDPOINT: &str = "https://inthetrenches.cloud:443";

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Outcome<T> {
    Success(T),
    Failure(String),
}
