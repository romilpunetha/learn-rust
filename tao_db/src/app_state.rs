use std::sync::Arc;
use crate::{
    database::TaoDatabase,
    services::{UserService, PostService, SocialService, GraphService},
    config::Config,
};

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub post_service: PostService,
    pub social_service: SocialService,
    pub graph_service: GraphService,
    pub config: Config,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        // Initialize database
        let database = TaoDatabase::new(&config.database.url, config.cache.capacity).await?;
        database.init().await?;
        let database = Arc::new(database);

        // Initialize services
        let user_service = UserService::new(database.clone());
        let post_service = PostService::new(database.clone());
        let social_service = SocialService::new(database.clone());
        
        // GraphService needs references to other services for proper layered architecture
        let user_service_arc = Arc::new(user_service.clone());
        let post_service_arc = Arc::new(post_service.clone());
        let social_service_arc = Arc::new(social_service.clone());
        let graph_service = GraphService::new(
            database.clone(),
            user_service_arc,
            social_service_arc,
            post_service_arc,
        );

        Ok(Self {
            user_service,
            post_service,
            social_service,
            graph_service,
            config,
        })
    }
}