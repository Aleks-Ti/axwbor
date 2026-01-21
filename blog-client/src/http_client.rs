use crate::error::BlogClientError;
use crate::grpc_auth;
use crate::grpc_blog;
use std::result::Result::Ok;

pub struct HttpClient {
    base_url: String,
    inner: reqwest::Client,
}

impl HttpClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            inner: reqwest::Client::new(),
        }
    }

    pub async fn register(
        &self,
        username: String,
        email: String,
        password: String,
    ) -> Result<grpc_auth::RegisterResponse, BlogClientError> {
        let body = grpc_auth::RegisterRequest {
            username,
            email,
            password,
        };
        let res = self
            .inner
            .post(format!("{}/auth/register", self.base_url))
            .json(&body)
            .send()
            .await?;
        if res.status() == 400 {
            let error_text = res.text().await.map_err(BlogClientError::Http)?;
            tracing::error!("Registration failed: {}", error_text);
            return Err(BlogClientError::InvalidRequest(error_text));
        }
        if res.status() == 409 {
            return Err(BlogClientError::NotUnique);
        }
        if res.status() == 404 {
            return Err(BlogClientError::NotFound);
        }
        let auth: grpc_auth::RegisterResponse = res.json().await?;
        Ok(auth)
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<grpc_auth::LoginResponse, BlogClientError> {
        let body = grpc_auth::LoginRequest { username, password };
        let res = self
            .inner
            .post(format!("{}/auth/login", self.base_url))
            .json(&body)
            .send()
            .await?;
        if res.status() == 400 {
            let error_text = res.text().await.map_err(BlogClientError::Http)?;
            tracing::error!("Login failed: {}", error_text);
            return Err(BlogClientError::InvalidRequest(error_text));
        }
        if res.status() == 401 {
            return Err(BlogClientError::Unauthorized);
        }
        if res.status() == 404 {
            return Err(BlogClientError::NotFound);
        }

        let auth: grpc_auth::LoginResponse = res.json().await?;
        Ok(auth)
    }

    pub async fn get_posts(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<grpc_blog::GetPostsResponse, BlogClientError> {
        let body = grpc_blog::GetPostsRequest { limit, offset };
        let res = self
            .inner
            .get(format!("{}/post", self.base_url))
            .query(&body)
            .send()
            .await?;
        if res.status() == 400 {
            let error_text = res.text().await.map_err(BlogClientError::Http)?;
            tracing::error!("Get Posts failed: {}", error_text);
            return Err(BlogClientError::InvalidRequest(error_text));
        }
        if res.status() == 401 {
            return Err(BlogClientError::Unauthorized);
        }
        if res.status() == 404 {
            return Err(BlogClientError::NotFound);
        }

        let posts: grpc_blog::GetPostsResponse = res.json().await?;
        Ok(posts)
    }

    pub async fn get_post(
        &self,
        post_id: i64,
    ) -> Result<grpc_blog::GetPostResponse, BlogClientError> {
        let res = self
            .inner
            .get(format!("{}/post/{}", self.base_url, post_id))
            .send()
            .await?;
        if res.status() == 400 {
            let error_text = res.text().await.map_err(BlogClientError::Http)?;
            tracing::error!("Get Post by id: {} - failed: {}", post_id, error_text);
            return Err(BlogClientError::InvalidRequest(error_text));
        }
        if res.status() == 401 {
            return Err(BlogClientError::Unauthorized);
        }
        if res.status() == 404 {
            return Err(BlogClientError::NotFound);
        }

        let post: grpc_blog::GetPostResponse = res.json().await?;
        Ok(post)
    }

    pub async fn update_post(
        &self,
        post_id: i64,
        title: Option<String>,
        content: Option<String>,
        token: String,
    ) -> Result<grpc_blog::UpdatePostResponse, BlogClientError> {
        let body = grpc_blog::UpdatePostRequest {
            id: post_id,
            title,
            content,
        };
        let res = self
            .inner
            .put(format!("{}/auth/post/{}", self.base_url, post_id))
            .header("access_token", token)
            .json(&body)
            .send()
            .await?;
        if res.status() == 400 {
            let error_text = res.text().await.map_err(BlogClientError::Http)?;
            tracing::error!("Update Post by id: {} - failed: {}", post_id, error_text);
            return Err(BlogClientError::InvalidRequest(error_text));
        }
        if res.status() == 401 {
            return Err(BlogClientError::Unauthorized);
        }
        if res.status() == 404 {
            return Err(BlogClientError::NotFound);
        }

        let post: grpc_blog::UpdatePostResponse = res.json().await?;
        Ok(post)
    }

    pub async fn create_post(
        &self,
        title: String,
        content: String,
        token: String,
    ) -> Result<grpc_blog::CreatePostResponse, BlogClientError> {
        let body = grpc_blog::CreatePostRequest { title, content };
        let res = self
            .inner
            .post(format!("{}/auth/post", self.base_url))
            .header("access_token", token)
            .json(&body)
            .send()
            .await?;
        if res.status() == 400 {
            let error_text = res.text().await.map_err(BlogClientError::Http)?;
            tracing::error!("Create Post - failed: {}", error_text);
            return Err(BlogClientError::InvalidRequest(error_text));
        }
        if res.status() == 401 {
            return Err(BlogClientError::Unauthorized);
        }
        if res.status() == 404 {
            return Err(BlogClientError::NotFound);
        }

        let post: grpc_blog::CreatePostResponse = res.json().await?;
        Ok(post)
    }

    pub async fn delete_post(&self, post_id: i64, token: String) -> Result<(), BlogClientError> {
        let res = self
            .inner
            .delete(format!("{}/auth/post/{}", self.base_url, post_id))
            .header("access_token", token)
            .send()
            .await?;
        if res.status() == 400 {
            let error_text = res.text().await.map_err(BlogClientError::Http)?;
            tracing::error!("Delete Post by id: {} - failed: {}", post_id, error_text);
            return Err(BlogClientError::InvalidRequest(error_text));
        }
        if res.status() == 401 {
            return Err(BlogClientError::Unauthorized);
        }
        if res.status() == 404 {
            return Err(BlogClientError::NotFound);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_register_success() {
        let mock_server = MockServer::start().await;
        let expected_email = "abc123@abc123.abc123";
        let expected_id = 42i64;

        Mock::given(method("POST"))
            .and(path("/auth/register"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(grpc_auth::RegisterResponse {
                    id: expected_id,
                    email: expected_email.to_string(),
                }),
            )
            .mount(&mock_server)
            .await;

        let client = HttpClient::new(mock_server.uri());
        let res = client
            .register(
                "user".to_string(),
                "test@test.com".to_string(),
                "pass".to_string(),
            )
            .await
            .expect("Register should succeed");

        assert_eq!(res.email, expected_email);
        assert_eq!(res.id, expected_id);
    }

    // Тест интеграционный, одноразовый. Чтобы работало, нужно запустить blog-server.
    // NOTE: добавить потом, очистку записи из БД
    #[tokio::test]
    async fn test_register() {
        let client = HttpClient::new("http://127.0.0.1:8080/api".to_string());
        let res = client
            .register(
                "username".to_string(),
                "email@email.email".to_string(),
                "password".to_string(),
            )
            .await;

        match res {
            Ok(auth) => {
                assert!(!auth.email.is_empty());
                assert!(!auth.id.is_negative());
            }
            Err(e) => {
                println!("IntegrationTest::Registration failed: {:?}", e);
            }
        }
    }

    /// Тест интеграционный, одноразовый. Чтобы работало, нужно запустить blog-server.
    /// NOTE: последовательный тест, зависит от [`tests::test_register`]
    #[tokio::test]
    async fn test_login() {
        let client = HttpClient::new("http://127.0.0.1:8080/api".to_string());
        let res = client
            .login("username".to_string(), "password".to_string())
            .await;

        match res {
            Ok(auth) => {
                assert!(!auth.access_token.is_empty());
            }
            Err(e) => {
                println!("IntegrationTest::Login failed: {:?}", e);
            }
        }
    }
}
