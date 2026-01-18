use crate::application::auth_service::AuthService;
use crate::grpc_auth::auth_service_server::AuthService as GrpcAuthService;
use crate::data::user_repository::UserRepository;
use crate::domain::error::AuthError;
use crate::grpc_auth::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use std::sync::Arc;
use tonic::{Request, Response, Status};

// Обёртка над PostService для gRPC
pub struct AuthGrpcService<R>
where
    R: UserRepository + 'static,
{
    auth_service: Arc<AuthService<R>>,
}

impl<R> AuthGrpcService<R>
where
    R: UserRepository + 'static,
{
    pub fn new(auth_service: Arc<AuthService<R>>) -> Self {
        Self { auth_service }
    }
}

// Маппинг ошибок
fn map_error(e: AuthError) -> Status {
    match e {
        AuthError::UserNotFound(_) => Status::not_found(e.to_string()),
        AuthError::Unauthorized => Status::unauthenticated(e.to_string()),
        AuthError::UserAlreadyExists(_) => Status::already_exists(e.to_string()),
        AuthError::Validation(_) => Status::invalid_argument(e.to_string()),
        AuthError::Internal(_) => Status::internal(e.to_string()),
    }
}

#[tonic::async_trait]
impl<R> GrpcAuthService for AuthGrpcService<R>
where
    R: crate::data::user_repository::UserRepository + Send + Sync + 'static,
{
    async fn register(
        &self,
        request: Request<RegisterRequest>,
    ) -> Result<Response<RegisterResponse>, Status> {
        let req = request.into_inner();
        let user = self
            .auth_service
            .register(req.email, req.username, req.password)
            .await
            .map_err(map_error)?;
        Ok(Response::new(RegisterResponse {
            id: user.id,
            email: user.email,
        }))
    }

    async fn login(
        &self,
        request: Request<LoginRequest>,
    ) -> Result<Response<LoginResponse>, Status> {
        let req = request.into_inner();
        let token = self
            .auth_service
            .login(&req.username, &req.password)
            .await
            .map_err(map_error)?;
        Ok(Response::new(LoginResponse {
            access_token: token,
        }))
    }
}
