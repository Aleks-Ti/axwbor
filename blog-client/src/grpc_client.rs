//! Blog Client gRPC Module

use crate::error::BlogClientError;
use crate::{grpc_auth, grpc_blog};
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::transport::Channel;

use crate::grpc_auth::auth_service_client::AuthServiceClient;
use crate::grpc_blog::post_service_client::PostServiceClient;

/// gRPC клиент для взаимодействия с блог-сервером.
pub struct GrpcClient {
    auth_client: AuthServiceClient<Channel>,
    blog_client: PostServiceClient<Channel>,
}

impl GrpcClient {
    /// Создаёт новый экземпляр GrpcClient с заданным каналом.
    pub fn new(channel: Channel) -> Self {
        Self {
            auth_client: AuthServiceClient::new(channel.clone()),
            blog_client: PostServiceClient::new(channel.clone()),
        }
    }

    /// Выполняет вход пользователя и возвращает ответ gRPC.
    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<grpc_auth::LoginResponse, BlogClientError> {
        let mut client = self.auth_client.clone();
        let request = tonic::Request::new(grpc_auth::LoginRequest { username, password });
        let response = client.login(request).await?.into_inner();
        Ok(response)
    }

    /// Регистрирует нового пользователя и возвращает ответ gRPC.
    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<grpc_auth::RegisterResponse, BlogClientError> {
        let mut client = self.auth_client.clone();
        let request = tonic::Request::new(grpc_auth::RegisterRequest {
            username,
            email,
            password,
        });
        let response = client.register(request).await?.into_inner();
        Ok(response)
    }

    /// Получает список постов с заданными лимитом и смещением.
    pub async fn get_posts(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<grpc_blog::GetPostsResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let request = tonic::Request::new(grpc_blog::GetPostsRequest { limit, offset });
        let response = client.get_posts(request).await?.into_inner();
        Ok(response)
    }

    /// Получает пост по его идентификатору.
    pub async fn get_post(
        &self,
        post_id: i64,
    ) -> Result<grpc_blog::GetPostResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let request = tonic::Request::new(grpc_blog::GetPostRequest { id: post_id });
        let response = client.get_post(request).await?.into_inner();
        Ok(response)
    }

    /// Создаёт новый пост с заданным заголовком и содержимым.
    pub async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<grpc_blog::CreatePostResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let mut meta = MetadataMap::new();
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))
            .map_err(|_| BlogClientError::InvalidRequest("ass".to_string()))?;
        meta.insert("authorization", token_value);
        let request = tonic::Request::from_parts(
            meta,
            tonic::Extensions::default(),
            grpc_blog::CreatePostRequest { title, content },
        );
        let response = client.create_post(request).await?.into_inner();
        Ok(response)
    }

    /// Обновляет пост с заданным идентификатором, заголовком и содержимым.
    pub async fn update_post(
        &self,
        post_id: i64,
        title: Option<String>,
        content: Option<String>,
        token: String,
    ) -> Result<grpc_blog::UpdatePostResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let mut meta = MetadataMap::new();
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))
            .map_err(|_| BlogClientError::InvalidRequest("ass".to_string()))?;
        meta.insert("authorization", token_value);
        let request = tonic::Request::from_parts(
            meta,
            tonic::Extensions::default(),
            grpc_blog::UpdatePostRequest {
                id: post_id,
                title,
                content,
            },
        );
        let response = client.update_post(request).await?.into_inner();
        Ok(response)
    }

    /// Удаляет пост с заданным идентификатором.
    pub async fn delete_post(
        &self,
        post_id: i64,
        token: String,
    ) -> Result<grpc_blog::DeletePostResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let mut meta = MetadataMap::new();
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))
            .map_err(|_| BlogClientError::InvalidRequest("ass".to_string()))?;
        meta.insert("authorization", token_value);
        let request = tonic::Request::from_parts(
            meta,
            tonic::Extensions::default(),
            grpc_blog::DeletePostRequest { id: post_id },
        );
        let response = client.delete_post(request).await?.into_inner();
        Ok(response)
    }
}
