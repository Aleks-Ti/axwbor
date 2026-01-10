use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DomainError {
    #[error("validation failed: {0}")]
    Validation(String),
    #[error("user already exists: {0}")]
    AlreadyExists(i32),
    #[error("account not found: {0}")]
    AccountNotFound(String),
    #[error("user not found: {0}")]
    NotFound(String),
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
    UserNotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("user already exists: {0}")]
    UserAlreadyExists(i32),
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
            AuthError::UserNotFound(_) => StatusCode::NOT_FOUND,
            AuthError::Unauthorized => StatusCode::UNAUTHORIZED,
            AuthError::UserAlreadyExists(_) => StatusCode::CONFLICT,
            AuthError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let details = match self {
            AuthError::Validation(msg) => Some(json!({ "message": msg })),
            AuthError::UserNotFound(resource) => Some(json!({ "resource": resource })),
            AuthError::Unauthorized => None,
            AuthError::UserAlreadyExists(account) => {
                Some(json!({ "account": account, "reason": "user_already_exists" }))
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
            DomainError::AlreadyExists(acc) => AuthError::UserAlreadyExists(acc),
            DomainError::AccountNotFound(acc) => {
                AuthError::UserNotFound(format!("account {}", acc))
            }
            DomainError::NotFound(id) => AuthError::UserNotFound(format!("user {}", id)),
            DomainError::Internal(msg) => AuthError::Internal(msg),
        }
    }
}

#[derive(Debug, Error)]
pub enum PostError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("post not found: {0}")]
    PostNotFound(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("internal server error: {0}")]
    Internal(String),
}

impl ResponseError for PostError {
    fn status_code(&self) -> StatusCode {
        match self {
            PostError::Validation(_) => StatusCode::BAD_REQUEST,
            PostError::PostNotFound(_) => StatusCode::NOT_FOUND,
            PostError::Unauthorized => StatusCode::UNAUTHORIZED,
            PostError::Forbidden => StatusCode::FORBIDDEN,
            PostError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let message = self.to_string();
        let details = match self {
            PostError::Validation(msg) => Some(json!({ "message": msg })),
            PostError::PostNotFound(resource) => Some(json!({ "resource": resource })),
            PostError::Unauthorized => None,
            PostError::Forbidden => None,
            PostError::Internal(_) => None,
        };
        let body = ErrorBody {
            error: &message,
            details,
        };
        HttpResponse::build(self.status_code()).json(body)
    }
}

impl From<DomainError> for PostError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::Unauthorized => PostError::Unauthorized,
            DomainError::Validation(msg) => PostError::Validation(msg),
            DomainError::NotFound(acc) => PostError::PostNotFound(acc),
            DomainError::Internal(msg) => PostError::Internal(msg),
            DomainError::AlreadyExists(_) => PostError::Internal("unexpected error".into()),
            DomainError::AccountNotFound(_) => PostError::Internal("unexpected error".into()),
        }
    }
}
