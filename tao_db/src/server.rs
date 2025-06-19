use axum::{http::Method, Router};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use tao_database::{AppState, Config};
use tao_database::routes::create_router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "tao_database=debug,tower_http=info,sqlx=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("🚀 Starting TAO Database Server");

    // Load configuration
    let config = Config::from_env();
    info!("📋 Configuration loaded: database_url={}", config.database.url);

    // Initialize application state
    let app_state = match AppState::new(config.clone()).await {
        Ok(state) => {
            info!("📊 Application state initialized successfully");
            state
        }
        Err(e) => {
            error!("❌ Failed to initialize application state: {}", e);
            return Err(e);
        }
    };

    // Configure CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    // Build the API router
    let api_router = create_router(app_state);

    // Serve static files from frontend directory
    let serve_dir = ServeDir::new("frontend/build");
    
    let app = Router::new()
        .nest("/api", api_router)
        .fallback_service(serve_dir)
        .layer(ServiceBuilder::new().layer(cors).into_inner());

    // Start the server
    let addr = config.server_address();
    info!("🌐 Server running on http://{}", addr);
    info!("📊 API available at http://{}/api", addr);
    info!("🎨 Frontend available at http://{}", addr);
    info!("💾 Health check: http://{}/api/health", addr);
    info!("🔗 Multi-layered architecture with proper error handling");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}