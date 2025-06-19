pub mod user_routes;
pub mod post_routes;
pub mod social_routes;
pub mod graph_routes;
pub mod health_routes;

use axum::{routing::get, Router};
use crate::AppState;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_routes::health_check))
        .merge(user_routes::create_user_router())
        .merge(post_routes::create_post_router())
        .merge(social_routes::create_social_router())
        .merge(graph_routes::create_graph_router())
        .with_state(state)
}