//! Blog Client Errors

use thiserror::Error;

/// Ошибки, которые могут возникнуть в клиенте блога.
#[derive(Error, Debug)]
pub enum BlogClientError {
    /// Ошибка HTTP-запроса.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Ошибка при сериализации или десериализации JSON.
    #[error("gRPC transport failed: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),

    /// Ошибка gRPC вызова.
    #[error("gRPC call failed: {0}")]
    GrpcStatus(#[from] tonic::Status),

    /// Ошибка ввода-вывода.
    #[error("Token is missing")]
    Io(#[from] std::io::Error),

    /// Ошибка отсутствия токена.
    #[error("Not found")]
    NotFound,

    /// Ошибка отсутствия токена.
    #[error("Unauthorized")]
    Unauthorized,

    /// Ошибка неуникального имени пользователя или email.
    #[error("Not unique username or email")]
    NotUnique,

    /// Некорректный запрос.
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    /// Ошибка отсутствия токена.
    #[error("Token is missing")]
    TokenMissing,
}
