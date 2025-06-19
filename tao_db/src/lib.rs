// TAO Database - Production-ready async implementation with SQLx and Thrift

// Core modules
pub mod cache;
pub mod database;
pub mod thrift_utils;
pub mod models;
pub mod viewer;

// Application architecture
pub mod error;
pub mod config;
pub mod dto;
pub mod app_state;
pub mod services;
pub mod routes;

// Re-exports for convenience
pub use app_state::AppState;
pub use config::Config;
pub use error::{AppError, AppResult};
