use axum::{
    extract::{Path, Query, State},
    routing::{get, post},
    Json, Router,
};

use crate::{
    dto::{CreateFriendshipRequest, CreateFollowRequest, CreateLikeRequest, 
          UserResponse, PostResponse, FriendsQuery, PostsQuery, ApiResponse},
    error::AppResult,
    viewer::ViewerContext,
    AppState,
};

pub fn create_social_router() -> Router<AppState> {
    Router::new()
        .route("/friendships", post(create_friendship))
        .route("/follows", post(create_follow))
        .route("/likes", post(create_like))
        .route("/users/{id}/friends", get(get_user_friends))
        .route("/users/{id}/posts", get(get_user_posts))
}

async fn create_friendship(
    State(state): State<AppState>,
    Json(req): Json<CreateFriendshipRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    state.social_service.create_friendship(req).await?;
    Ok(Json(ApiResponse::success_message("Friendship created successfully".to_string())))
}

async fn create_follow(
    State(state): State<AppState>,
    Json(req): Json<CreateFollowRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    state.social_service.create_follow(req).await?;
    Ok(Json(ApiResponse::success_message("Follow created successfully".to_string())))
}

async fn create_like(
    State(state): State<AppState>,
    Json(req): Json<CreateLikeRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    state.social_service.create_like(req).await?;
    Ok(Json(ApiResponse::success_message("Like created successfully".to_string())))
}

async fn get_user_friends(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Query(params): Query<FriendsQuery>,
) -> AppResult<Json<ApiResponse<Vec<UserResponse>>>> {
    let viewer_id = params.viewer_id.unwrap_or(user_id);
    let viewer = ViewerContext::new(viewer_id);
    
    let friends = state.social_service.get_friends(&viewer, user_id, params.limit).await?;
    Ok(Json(ApiResponse::success(friends)))
}

async fn get_user_posts(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    Query(params): Query<PostsQuery>,
) -> AppResult<Json<ApiResponse<Vec<PostResponse>>>> {
    let viewer_id = params.viewer_id.unwrap_or(user_id);
    let viewer = ViewerContext::new(viewer_id);
    
    let posts = state.social_service.get_posts_by_user(&viewer, user_id, params.limit).await?;
    Ok(Json(ApiResponse::success(posts)))
}