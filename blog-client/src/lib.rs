pub mod grpc_blog {
    tonic::include_proto!("blog");
}

pub mod grpc_auth {
    tonic::include_proto!("auth");
}

pub mod error;
pub mod grpc_client;
pub mod http_client;
pub mod models;

pub use error::BlogClientError;
// pub use grpc_client::GrpcClient;
pub use http_client::HttpClient;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Transport {
    Http(String), // base URL "http://localhost:8080"
    Grpc(String), // base URL "http://localhost:50051"
}

#[derive(Clone)]
pub struct BlogClient {
    transport: Transport,
    http_client: Option<Arc<reqwest::Client>>,
    grpc_client: Option<tonic::client::Grpc<tonic::transport::Channel>>,
    token: Option<String>,
}


impl BlogClient {
    pub async fn new(transport: Transport) -> Result<Self, BlogClientError> {
        let mut client = Self {
            transport,
            http_client: None,
            grpc_client: None,
            token: None,
        };

        match &client.transport {
            Transport::Http(base_url) => {
                let http = reqwest::Client::new();
                client.http_client = Some(Arc::new(http));
            }
            Transport::Grpc(addr) => {
                let channel = tonic::transport::Endpoint::from_shared(addr.clone())
                    .map_err(|e| BlogClientError::GrpcTransport(e.into()))?
                    .connect()
                    .await?;
                client.grpc_client = Some(tonic::client::Grpc::new(channel));
            }
        }

        Ok(client)
    }

    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    // дальше остальные методы.
}
