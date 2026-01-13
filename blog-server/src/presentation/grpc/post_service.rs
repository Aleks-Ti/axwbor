use crate::application::auth_service::AuthService;
use crate::application::blog_service::PostService;
use crate::data::post_repository::PostRepository;
use crate::data::user_repository::{UserRepository};
use crate::domain::error::PostError;
use crate::post_service_server::PostService as GrpcPostService;
use crate::presentation::auth::{JwtIdentity, extract_identity_from_token};
use crate::{
    CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest,
    GetPostResponse, GetPostsRequest, GetPostsResponse, Post as GrpcPost, UpdatePostRequest,
    UpdatePostResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

// Обёртка над PostService для gRPC
pub struct PostGrpcService<R, U>
where
    R: PostRepository + 'static,
    U: UserRepository + 'static,
{
    post_service: Arc<PostService<R>>,
    auth_service: Arc<AuthService<U>>,
}

impl<R, U> PostGrpcService<R, U>
where
    R: PostRepository + 'static,
    U: UserRepository + 'static,
{
    pub fn new(
        post_service: Arc<PostService<R>>,
        auth_service: Arc<AuthService<U>>,
    ) -> Self {
        Self {
            post_service,
            auth_service,
        }
    }
    async fn extract_identity_from_request(
        &self,
        request: &Request<impl std::fmt::Debug>,
    ) -> Result<JwtIdentity, Status> {
        let auth_header = request
            .metadata()
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or_else(|| Status::unauthenticated("authorization required"))?;

        let identity = extract_identity_from_token(auth_header, &self.auth_service.keys())
            .map_err(|_| Status::unauthenticated("invalid token"))?;

        Ok(identity)
    }
}

// Конвертеры
fn domain_to_grpc(post: crate::domain::post::Post) -> GrpcPost {
    GrpcPost {
        id: post.id,
        title: post.title,
        content: post.content,
        author_id: post.author_id,
        created_at: post.created_at.to_rfc3339(), // ISO строка
    }
}

// Маппинг ошибок
fn map_error(e: PostError) -> Status {
    match e {
        PostError::PostNotFound(_) => Status::not_found(e.to_string()),
        PostError::Unauthorized | PostError::Forbidden => Status::permission_denied(e.to_string()),
        PostError::Validation(_) => Status::invalid_argument(e.to_string()),
        PostError::Internal(_) => Status::internal(e.to_string()),
    }
}

#[tonic::async_trait]
impl<R, U> GrpcPostService for PostGrpcService<R, U>
where
    R: crate::data::post_repository::PostRepository + Send + Sync + 'static,
    U: UserRepository + Send + Sync + 'static,
{
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        let identity = self.extract_identity_from_request(&request).await?;

        let req = request.into_inner();
        let user = self
            .auth_service
            .get_user(identity.user_id)
            .await
            .map_err(|_| Status::unauthenticated("user not found"))?;
        let post = self
            .post_service
            .create_post(req.title, req.content, user.id)
            .await
            .map_err(map_error)?;
        Ok(Response::new(CreatePostResponse {
            post: Some(domain_to_grpc(post)),
        }))
    }

    async fn get_posts(
        &self,
        _request: Request<GetPostsRequest>,
    ) -> Result<Response<GetPostsResponse>, Status> {
        let posts = self.post_service.get_posts().await.map_err(map_error)?;
        let grpc_posts = posts.into_iter().map(domain_to_grpc).collect();
        Ok(Response::new(GetPostsResponse { posts: grpc_posts }))
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        let id = request.into_inner().id;
        let post = self.post_service.get_post(id).await.map_err(map_error)?;
        Ok(Response::new(GetPostResponse {
            post: Some(domain_to_grpc(post)),
        }))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<UpdatePostResponse>, Status> {
        let identity = self.extract_identity_from_request(&request).await?;

        let req = request.into_inner();
        let user = self
            .auth_service
            .get_user(identity.user_id)
            .await
            .map_err(|_| Status::unauthenticated("user not found"))?;
        let post = self
            .post_service
            .update_post(req.id, req.title, req.content, user.id)
            .await
            .map_err(map_error)?;
        Ok(Response::new(UpdatePostResponse {
            post: Some(domain_to_grpc(post)),
        }))
    }

    async fn delete_post(
        &self,
        request: Request<DeletePostRequest>,
    ) -> Result<Response<DeletePostResponse>, Status> {
        let identity = self.extract_identity_from_request(&request).await?;

        let id = request.into_inner().id;
        let user = self
            .auth_service
            .get_user(identity.user_id)
            .await
            .map_err(|_| Status::unauthenticated("user not found"))?;
        self.post_service
            .delete_post(id, user.id)
            .await
            .map_err(|e| {
                if matches!(e, PostError::Forbidden) {
                    Status::permission_denied("you are not the author")
                } else {
                    map_error(e)
                }
            })?;
        Ok(Response::new(DeletePostResponse {}))
    }
}
