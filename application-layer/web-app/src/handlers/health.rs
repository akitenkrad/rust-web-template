use crate::models::health::HealthCheckResponse;
use axum::{
    extract::{Json, State},
    http::StatusCode,
};
use registry::AppRegistry;
use shared::errors::AppResult;

pub async fn health_check() -> AppResult<Json<HealthCheckResponse>> {
    tracing::info!("Health check endpoint hit");
    Ok(Json(HealthCheckResponse::from(StatusCode::OK)))
}

pub async fn health_check_db(
    State(registry): State<AppRegistry>,
) -> AppResult<Json<HealthCheckResponse>> {
    if registry.health_check_repository().check_db().await {
        Ok(Json(HealthCheckResponse::from(StatusCode::OK)))
    } else {
        Ok(Json(HealthCheckResponse::from(
            StatusCode::INTERNAL_SERVER_ERROR,
        )))
    }
}
