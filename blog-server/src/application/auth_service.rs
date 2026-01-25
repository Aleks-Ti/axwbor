use std::sync::Arc;

use tracing::instrument;

use crate::data::user_repository::UserRepository;
use crate::domain::{error::AuthError, error::DomainError, user::NewUser, user::User};
use crate::infrastructure::jwt::{JwtKeys, hash_password, verify_password};

#[derive(Clone)]
pub struct AuthService<R: UserRepository + 'static> {
    repo: Arc<R>,
    keys: JwtKeys,
}

impl<R> AuthService<R>
where
    R: UserRepository + 'static,
{
    /// Создаёт новый экземпляр AuthService.
    pub fn new(repo: Arc<R>, keys: JwtKeys) -> Self {
        Self { repo, keys }
    }

    /// Получение JWT ключей.
    pub fn keys(&self) -> &JwtKeys {
        &self.keys
    }

    /// Получение пользователя по ID.
    pub async fn get_user(&self, id: i64) -> Result<User, AuthError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or(AuthError::UserNotFound(format!("user {}", id)))
    }

    /// Регистрация нового пользователя.
    #[instrument(skip(self))]
    pub async fn register(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<User, AuthError> {
        let hash = hash_password(&password).map_err(|err| AuthError::Internal(err.to_string()))?;
        let user = NewUser::new(email.to_lowercase(), username.to_lowercase(), hash);
        self.repo.create(user).await
    }

    /// Вход пользователя и генерация JWT токена.
    #[instrument(skip(self))]
    pub async fn login(&self, username: &str, password: &str) -> Result<String, AuthError> {
        let user = self
            .repo
            .find_by_username(&username.to_lowercase())
            .await?
            .ok_or(DomainError::Unauthorized)?;
        let valid = verify_password(password, &user.password_hash)
            .map_err(|_| DomainError::Unauthorized)?;
        if !valid {
            return Err(AuthError::Unauthorized);
        }
        self.keys
            .generate_token(user.id)
            .map_err(|err| AuthError::Internal(err.to_string()))
    }
}
