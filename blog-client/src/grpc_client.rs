// use crate::models::{Post, RegisterRequest, RegisterResponse, User};
// use crate::error::BlogClientError;
// use tonic::transport::Channel;

// use crate::grpc_auth::auth_service_client::AuthServiceClient;
// use crate::grpc_blog::post_service_client::PostServiceClient;

// pub struct GrpcClient {
//     auth_client: AuthServiceClient<Channel>,
//     blog_client: PostServiceClient<Channel>,
// }

// impl GrpcClient {
//     pub fn new(channel: Channel) -> Self {
//         Self {
//             auth_client: AuthServiceClient::new(channel.clone()),
//             blog_client: PostServiceClient::new(channel),
//         }
//     }

//     pub async fn login(
//         &mut self,
//         email: &str,
//         password: &str,
//     ) -> Result<AuthResponse, BlogClientError> {
//         let request = tonic::Request::new(crate::grpc_auth::LoginRequest {
//             email: email.to_string(),
//             password: password.to_string(),
//         });

//         let response = self.auth_client.login(request).await?.into_inner();

//         Ok(AuthResponse {
//             token: response.token,
//         })
//     }
// }
