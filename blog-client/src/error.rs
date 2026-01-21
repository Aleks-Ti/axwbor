use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlogClientError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("gRPC transport failed: {0}")]
    GrpcTransport(#[from] tonic::transport::Error),

    #[error("gRPC call failed: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("Token is missing")]
    Io(#[from] std::io::Error),

    #[error("Not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Not unique username or email")]
    NotUnique,

    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Token is missing")]
    TokenMissing,
}
