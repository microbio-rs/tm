use axum::{
    extract::{rejection::FormRejection, FromRequest, Path, Request},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
    Form, Router,
};
use serde::{de::DeserializeOwned, Deserialize};
use thiserror::Error;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct OkrForm {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub objective: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub owner: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub start_date: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub end_date: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedForm<T>(pub T);

#[axum::async_trait]
impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
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
    ValidatedForm(_okr): ValidatedForm<OkrForm>,
    // state: axum::Extension<SharedState>,
) -> impl IntoResponse {
    Redirect::to("/okrs/1/add-key-results").into_response()
}

/// Handler for add key results for OKR
async fn add_key_results(Path(okr_id): Path<u32>) -> impl IntoResponse {
    format!("add-key-result to okr id {okr_id}")
}
