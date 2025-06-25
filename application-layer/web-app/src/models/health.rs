use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct HealthCheckResponse {
    pub message: String,
    pub status_code: usize,
}

impl From<StatusCode> for HealthCheckResponse {
    fn from(status: StatusCode) -> Self {
        if status == StatusCode::OK {
            Self {
                message: String::from("OK"),
                status_code: status.as_u16() as usize,
            }
        } else {
            Self {
                message: String::from("Health check failed"),
                status_code: status.as_u16() as usize,
            }
        }
    }
}
