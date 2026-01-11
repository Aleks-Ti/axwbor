use crate::application::blog_service::PostService;
use crate::data::post_repository::PostRepository;
use crate::domain::error::PostError;
use crate::domain::post::NewPost;
use crate::post_service_server::PostService as GrpcPostService;
use crate::{
    CreatePostRequest, CreatePostResponse, DeletePostRequest, DeletePostResponse, GetPostRequest,
    GetPostResponse, GetPostsRequest, GetPostsResponse, Post as GrpcPost, UpdatePostRequest,
    UpdatePostResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};

// Обёртка над PostService для gRPC
pub struct PostGrpcService<R>
where
    R: PostRepository + 'static,
{
    service: Arc<PostService<R>>,
}

impl<R> PostGrpcService<R>
where
    R: crate::data::post_repository::PostRepository + 'static,
{
    pub fn new(service: Arc<PostService<R>>) -> Self {
        Self { service }
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

fn grpc_to_domain_new_post(req: &CreatePostRequest) -> NewPost {
    NewPost::new(req.title.clone(), req.content.clone(), req.author_id)
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
impl<R> GrpcPostService for PostGrpcService<R>
where
    R: crate::data::post_repository::PostRepository + Send + Sync + 'static,
{
    async fn create_post(
        &self,
        request: Request<CreatePostRequest>,
    ) -> Result<Response<CreatePostResponse>, Status> {
        let req = request.into_inner();
        let new_post = grpc_to_domain_new_post(&req);
        let post = self
            .service
            .create_post(new_post.title, new_post.content, new_post.author_id)
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
        let posts = self.service.get_posts().await.map_err(map_error)?;
        let grpc_posts = posts.into_iter().map(domain_to_grpc).collect();
        Ok(Response::new(GetPostsResponse { posts: grpc_posts }))
    }

    async fn get_post(
        &self,
        request: Request<GetPostRequest>,
    ) -> Result<Response<GetPostResponse>, Status> {
        let id = request.into_inner().id;
        let post = self.service.get_post(id).await.map_err(map_error)?;
        Ok(Response::new(GetPostResponse {
            post: Some(domain_to_grpc(post)),
        }))
    }

    async fn update_post(
        &self,
        request: Request<UpdatePostRequest>,
    ) -> Result<Response<UpdatePostResponse>, Status> {
        let req = request.into_inner();
        let current_user = crate::presentation::auth::AuthenticatedUser {
            id: req.author_id,
            email: "stub@example.com".to_string(), // stub, не используется в update/delete логике кроме проверки id
        };
        let post = self
            .service
            .update_post(req.id, req.title, req.content, current_user)
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
        let id = request.into_inner().id;
        let current_user = crate::presentation::auth::AuthenticatedUser {
            id: 0, // ← проблема! Нужен реальный user_id
            email: "stub@example.com".to_string(),
        };
        self.service
            .delete_post(id, current_user)
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
