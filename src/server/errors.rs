use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use diesel::result::Error as DieselError;
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    ValidationFailed(String),
    DbError(String),
    Unauthorized(String),
    HashError,
    PoolError,
}

impl From<DieselError> for ApiError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => ApiError::NotFound("Record not found".into()),
            _ => ApiError::DbError("Database error".into()),
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(_: ValidationErrors) -> Self {
        ApiError::ValidationFailed("Invalid payload".into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::ValidationFailed(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::DbError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::HashError => (StatusCode::INTERNAL_SERVER_ERROR, "Hashing failed".into()),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::PoolError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database unavailable".into(),
            ),
        };

        let body = Json(json!({ "error": msg }));
        (status, body).into_response()
    }
}
