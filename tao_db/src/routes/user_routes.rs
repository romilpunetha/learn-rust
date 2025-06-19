use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    Json, Router,
};

use crate::{
    dto::{CreateUserRequest, UpdateUserRequest, UserResponse, UserStats, UserQuery, ApiResponse},
    error::AppResult,
    AppState,
};

pub fn create_user_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_all_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .route("/users/{id}/stats", get(get_user_stats))
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user = state.user_service.create_user(req).await?;
    Ok(Json(ApiResponse::success(user)))
}

async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user = state.user_service.get_user(user_id).await?;
    Ok(Json(ApiResponse::success(user)))
}

async fn get_all_users(
    State(state): State<AppState>,
    Query(params): Query<UserQuery>,
) -> AppResult<Json<ApiResponse<Vec<UserResponse>>>> {
    let users = state.user_service.get_all_users(params.limit).await?;
    Ok(Json(ApiResponse::success(users)))
}

async fn update_user(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Json(req): Json<UpdateUserRequest>,
) -> AppResult<Json<ApiResponse<UserResponse>>> {
    let user = state.user_service.update_user(user_id, req).await?;
    Ok(Json(ApiResponse::success(user)))
}

async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> AppResult<Json<ApiResponse<()>>> {
    state.user_service.delete_user(user_id).await?;
    Ok(Json(ApiResponse::success_message("User deleted successfully".to_string())))
}

async fn get_user_stats(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
) -> AppResult<Json<ApiResponse<UserStats>>> {
    let stats = state.user_service.get_user_stats(user_id).await?;
    Ok(Json(ApiResponse::success(stats)))
}