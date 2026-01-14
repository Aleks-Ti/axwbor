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
    http_client: Option<Arc<crate::http_client::HttpClient>>,
    grpc_client: Option<tonic::client::Grpc<tonic::transport::Channel>>,
    token: Option<String>,
}

impl BlogClient {
    pub async fn login(
        &mut self,
        username: String,
        password: String,
    ) -> Result<(), BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let http = self.http_client.as_ref().unwrap();
                let token_resp = http.login(username, password).await?;
                self.set_token(token_resp.access_token);
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
    ) -> Result<(), BlogClientError> {
        match &self.transport {
            Transport::Http(_) => {
                let http = self.http_client.as_ref().unwrap();
                http.register(username, email, password).await?;
                Ok(())
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

    // дальше остальные методы.
}
