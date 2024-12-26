use axum::{
    extract::Path,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OkrForm {
    pub objective: String,
    pub owner: String,
    pub start_date: String,
    pub end_date: String,
}

/// Creates a router with the required routes
pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(|| async {}))
        .route("/okrs", post(create_okr))
        .route("/okrs/:okr_id/add-key-results", get(add_key_results))
}

/// Handler for creating OKRs with validation
async fn create_okr(
    Form(_okr): Form<OkrForm>,
    // state: axum::Extension<SharedState>,
) -> impl IntoResponse {
    Redirect::to("/okrs/1/add-key-results").into_response()
}

/// Handler for add key results for OKR
async fn add_key_results(Path(okr_id): Path<u32>) -> impl IntoResponse {
    format!("add-key-result to okr id {okr_id}")
}
