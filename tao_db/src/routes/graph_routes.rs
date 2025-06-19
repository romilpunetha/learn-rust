use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};

use crate::{
    dto::{GraphData, GraphQuery, ApiResponse},
    error::AppResult,
    viewer::ViewerContext,
    AppState,
};

pub fn create_graph_router() -> Router<AppState> {
    Router::new()
        .route("/graph", get(get_graph_data))
        .route("/seed", post(seed_data))
}

async fn get_graph_data(
    State(state): State<AppState>,
    Query(params): Query<GraphQuery>,
) -> AppResult<Json<ApiResponse<GraphData>>> {
    let max_users = params.max_users.unwrap_or(20);
    let viewer_id = params.viewer_id.unwrap_or(1);
    let viewer = ViewerContext::new(viewer_id);
    
    let graph_data = state.graph_service.get_social_graph_data(&viewer, max_users).await?;
    Ok(Json(ApiResponse::success(graph_data)))
}

async fn seed_data(
    State(state): State<AppState>,
) -> AppResult<Json<ApiResponse<()>>> {
    state.graph_service.seed_sample_data().await?;
    Ok(Json(ApiResponse::success_message("Sample data seeded successfully".to_string())))
}