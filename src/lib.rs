use axum::{routing::get, Router};

/// Creates a router with the required routes
pub fn create_router() -> Router {
    Router::new().route("/health", get(|| async {}))
}
