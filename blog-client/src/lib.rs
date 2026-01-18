pub mod grpc_blog {
    tonic::include_proto!("blog");
}

pub mod grpc_auth {
    tonic::include_proto!("auth");
}

pub mod error;
pub mod grpc_client;
pub mod http_client;
pub use error::BlogClientError;
pub use http_client::HttpClient;

use std::sync::Arc;

pub const DEFAULT_HTTP_URL: &str = "http://localhost:8080";
pub const DEFAULT_GRPC_URL: &str = "http://localhost:50051";

// Выбор транспорта для общение с API
// Можно передать как дефолт let client = BlogClient::new(Transport::http_default()).await?;
// или let client = BlogClient::new(Transport::Http("https://api.myblog.com".into())).await?;
#[derive(Debug, Clone)]
pub enum Transport {
    Http(String),
    Grpc(String),
}

impl Transport {
    pub fn http_default() -> Self {
        Self::Http(DEFAULT_HTTP_URL.to_string())
    }

    pub fn grpc_default() -> Self {
        Self::Grpc(DEFAULT_GRPC_URL.to_string())
    }
}

#[derive(Clone)]
pub struct BlogClient {
    transport: Transport,
    http_client: Option<Arc<crate::http_client::HttpClient>>,
    grpc_client: Option<Arc<crate::grpc_client::GrpcClient>>,
    token: Option<String>,
}

impl BlogClient {
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        let http_client = match &transport {
            Transport::Http(base_url) => Some(Arc::new(HttpClient::new(base_url.clone()))),
            Transport::Grpc(_) => None,
        };

        let grpc_client = match &transport {
            Transport::Http(_) => None,
            Transport::Grpc(addr) => {
                let channel = tonic::transport::Endpoint::from_shared(addr.clone())
                    .map_err(|e| BlogClientError::GrpcTransport(e.into()))?
                    .connect()
                    .await?;
                Some(Arc::new(crate::grpc_client::GrpcClient::new(channel)))
            }
        };

        Ok(Self {
            transport,
            http_client,
            grpc_client,
            token: None,
        })
    }

    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let client = self
                    .http_client
                    .as_ref()
                    .expect("HTTP client not initialized");
                let response = client.login(username, password).await?;
                self.set_token(response.access_token);
                Ok(())
            }
            Transport::Grpc(_) => {
                // NOTE: вызов gRPC
                unimplemented!()
            }
        }
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<grpc_auth::RegisterResponse, BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let client = self
                    .http_client
                    .as_ref()
                    .expect("HTTP client not initialized");
                let response = client.register(username, email, password).await?;
                Ok(response)
            }
            Transport::Grpc(_) => unimplemented!(),
        }
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub async fn create_post(
        &self,
        title: String,
        content: String,
    ) -> Result<grpc_blog::CreatePostResponse, BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let client = self
                    .http_client
                    .as_ref()
                    .expect("HTTP client not initialized");
                let token = self.get_token().ok_or(BlogClientError::TokenMissing)?;
                let response = client
                    .create_post(title, content, token.to_string())
                    .await?;
                Ok(response)
            }
            Transport::Grpc(_) => {
                let client = self
                    .grpc_client
                    .as_ref()
                    .expect("gRPC client not initialized")
                    .clone();
                let token = self
                    .get_token()
                    .ok_or(BlogClientError::TokenMissing)?
                    .to_string();
                let response = client.create_post(title, content, token).await?;
                Ok(response)
            }
        }
    }

    pub async fn update_post(
        &self,
        post_id: i64,
        title: String,
        content: String,
    ) -> Result<grpc_blog::UpdatePostResponse, BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let client = self
                    .http_client
                    .as_ref()
                    .expect("HTTP client not initialized");
                let token = self.get_token().ok_or(BlogClientError::TokenMissing)?;
                let response = client
                    .update_post(post_id, title, content, token.to_string())
                    .await?;
                Ok(response)
            }
            Transport::Grpc(_) => {
                let client = self
                    .grpc_client
                    .as_ref()
                    .expect("gRPC client not initialized")
                    .clone();
                let token = self
                    .get_token()
                    .ok_or(BlogClientError::TokenMissing)?
                    .to_string();
                let response = client.update_post(post_id, title, content, token).await?;
                Ok(response)
            }
        }
    }

    pub async fn delete_post(&self, post_id: i64) -> Result<(), BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let client = self
                    .http_client
                    .as_ref()
                    .expect("HTTP client not initialized");
                let token = self.get_token().ok_or(BlogClientError::TokenMissing)?;
                client.delete_post(post_id, token.to_string()).await?;
                Ok(())
            }
            Transport::Grpc(_) => {
                let client = self
                    .grpc_client
                    .as_ref()
                    .expect("gRPC client not initialized")
                    .clone();
                let token = self
                    .get_token()
                    .ok_or(BlogClientError::TokenMissing)?
                    .to_string();
                client.delete_post(post_id, token).await?;
                Ok(())
            }
        }
    }

    pub async fn list_posts(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<grpc_blog::GetPostsResponse, BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let client = self
                    .http_client
                    .as_ref()
                    .expect("HTTP client not initialized");
                let response = client.get_posts(limit, offset).await?;
                Ok(response)
            }
            Transport::Grpc(_) => {
                let client = self
                    .grpc_client
                    .as_ref()
                    .expect("gRPC client not initialized")
                    .clone();
                let response = client.get_posts(limit, offset).await?;
                Ok(response)
            }
        }
    }
    pub async fn get_post(
        &self,
        post_id: i64,
    ) -> Result<grpc_blog::GetPostResponse, BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let client = self
                    .http_client
                    .as_ref()
                    .expect("HTTP client not initialized");
                let response = client.get_post(post_id).await?;
                Ok(response)
            }
            Transport::Grpc(_) => {
                let client = self
                    .grpc_client
                    .as_ref()
                    .expect("gRPC client not initialized")
                    .clone();
                let response = client.get_post(post_id).await?;
                Ok(response)
            }
        }
    }
}
