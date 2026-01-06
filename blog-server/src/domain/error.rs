use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DomainError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("insufficient funds on account {0}")]
    InsufficientFunds(i32),
    #[error("account not found: {0}")]
    AccountNotFound(i32),
    #[error("user not found: {0}")]
    UserNotFound(Uuid),
    #[error("internal error: {0}")]
    Internal(String),
    #[error("unauthorized")]
    Unauthorized,
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("insufficient funds on account {0}")]
    InsufficientFunds(i32),
    #[error("internal server error: {0}")]
    Internal(String),
}

#[derive(Serialize)]
struct ErrorBody<'a> {
    error: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::Validation(_) => StatusCode::BAD_REQUEST,
            AuthError::NotFound(_) => StatusCode::NOT_FOUND,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::InsufficientFunds(_) => StatusCode::BAD_REQUEST,
            AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let details = match self {
            AuthError::Validation(msg) => Some(json!({ "message": msg })),
            AuthError::NotFound(resource) => Some(json!({ "resource": resource })),
            AuthError::Unauthorized => None,
            AuthError::InsufficientFunds(account) => {
                Some(json!({ "account_id": account, "reason": "insufficient_funds" }))
            }
            AuthError::Internal(_) => None,
        };
        let body = ErrorBody {
            error: &message,
            details,
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}

impl From<DomainError> for AuthError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::Unauthorized => AuthError::Unauthorized,
            DomainError::Validation(msg) => AuthError::Validation(msg),
            DomainError::InsufficientFunds(acc) => AuthError::InsufficientFunds(acc),
            DomainError::AccountNotFound(acc) => AuthError::NotFound(format!("account {}", acc)),
            DomainError::UserNotFound(id) => AuthError::NotFound(format!("user {}", id)),
            DomainError::Internal(msg) => AuthError::Internal(msg),
        }
    }
}
