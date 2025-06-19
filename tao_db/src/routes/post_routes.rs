use axum::{
    extract::{Path, State},
    routing::{get, post, delete},
    Json, Router,
};

use crate::{
    dto::{CreatePostRequest, PostResponse, ApiResponse},
    error::AppResult,
    AppState,
};

pub fn create_post_router() -> Router<AppState> {
    Router::new()
        .route("/posts", post(create_post))
        .route("/posts/{id}", get(get_post).delete(delete_post))
}

async fn create_post(
    State(state): State<AppState>,
    Json(req): Json<CreatePostRequest>,
) -> AppResult<Json<ApiResponse<PostResponse>>> {
    let post = state.post_service.create_post(req).await?;
    Ok(Json(ApiResponse::success(post)))
}

async fn get_post(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
) -> AppResult<Json<ApiResponse<PostResponse>>> {
    let post = state.post_service.get_post(post_id).await?;
    Ok(Json(ApiResponse::success(post)))
}

async fn delete_post(
    State(state): State<AppState>,
    Path(post_id): Path<i64>,
) -> AppResult<Json<ApiResponse<()>>> {
    state.post_service.delete_post(post_id).await?;
    Ok(Json(ApiResponse::success_message("Post deleted successfully".to_string())))
}