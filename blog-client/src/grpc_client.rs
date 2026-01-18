use crate::error::BlogClientError;
use crate::{grpc_auth, grpc_blog};
use tonic::metadata::{MetadataMap, MetadataValue};
use tonic::transport::Channel;

use crate::grpc_auth::auth_service_client::AuthServiceClient;
use crate::grpc_blog::post_service_client::PostServiceClient;

pub struct GrpcClient {
    auth_client: AuthServiceClient<Channel>,
    blog_client: PostServiceClient<Channel>,
}

impl GrpcClient {
    pub fn new(channel: Channel) -> Self {
        Self {
            auth_client: AuthServiceClient::new(channel.clone()),
            blog_client: PostServiceClient::new(channel.clone()),
        }
    }

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

    pub async fn register(
        &self,
        email: String,
        username: String,
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
    pub async fn get_post(
        &self,
        post_id: i64,
    ) -> Result<grpc_blog::GetPostResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let request = tonic::Request::new(grpc_blog::GetPostRequest { id: post_id });
        let response = client.get_post(request).await?.into_inner();
        Ok(response)
    }

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
        meta.insert("access_token", token_value);
        let request = tonic::Request::from_parts(
            meta,
            tonic::Extensions::default(),
            grpc_blog::CreatePostRequest { title, content },
        );
        let response = client.create_post(request).await?.into_inner();
        Ok(response)
    }
    pub async fn update_post(
        &self,
        post_id: i64,
        title: String,
        content: String,
        token: String,
    ) -> Result<grpc_blog::UpdatePostResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let mut meta = MetadataMap::new();
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))
            .map_err(|_| BlogClientError::InvalidRequest("ass".to_string()))?;
        meta.insert("access_token", token_value);
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
    pub async fn delete_post(
        &self,
        post_id: i64,
        token: String,
    ) -> Result<grpc_blog::DeletePostResponse, BlogClientError> {
        let mut client = self.blog_client.clone();
        let mut meta = MetadataMap::new();
        let token_value = MetadataValue::try_from(format!("Bearer {}", token))
            .map_err(|_| BlogClientError::InvalidRequest("ass".to_string()))?;
        meta.insert("access_token", token_value);
        let request = tonic::Request::from_parts(
            meta,
            tonic::Extensions::default(),
            grpc_blog::DeletePostRequest { id: post_id },
        );
        let response = client.delete_post(request).await?.into_inner();
        Ok(response)
    }
}
