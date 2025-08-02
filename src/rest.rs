use axum::{routing::get, Router};

pub async fn root() -> &'static str {
    "Hello, world!"
}

pub async fn readiness() -> &'static str {
    "OK"
}

pub async fn liveness() -> &'static str {
    "OK"
}

// Create the application router
pub fn create_router() -> Router {
    Router::new()
        .route("/api", get(root))
        .route("/readiness", get(readiness))
        .route("/liveness", get(liveness))
}