use axum::Json;
use crate::dto::ApiResponse;

pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success("TAO Database API is running".to_string()))
}