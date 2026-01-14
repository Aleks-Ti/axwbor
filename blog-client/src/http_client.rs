use crate::BlogClient;
use crate::error::BlogClientError;
use crate::models::{LoginRequest, Post, RegisterRequest, RegisterResponse, TokenResponse, User};
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
    ) -> Result<RegisterResponse, BlogClientError> {
        let body = RegisterRequest {
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
        let auth: RegisterResponse = res.json().await?;
        Ok(auth)
    }

    pub async fn login(
        &self,
        username: String,
        password: String,
    ) -> Result<TokenResponse, BlogClientError> {
        let body = LoginRequest { username, password };
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

        let auth: TokenResponse = res.json().await?;
        Ok(auth)
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
            .respond_with(ResponseTemplate::new(200).set_body_json(RegisterResponse {
                user_id: expected_id,
                email: expected_email.to_string(),
            }))
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
        assert_eq!(res.user_id, expected_id);
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
                assert!(!auth.user_id.is_negative());
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
