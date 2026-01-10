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
    pub fn new(repo: Arc<R>, keys: JwtKeys) -> Self {
        Self { repo, keys }
    }

    pub fn keys(&self) -> &JwtKeys {
        &self.keys
    }

    pub async fn get_user(&self, id: i64) -> Result<User, AuthError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(AuthError::from)?
            .ok_or_else(|| AuthError::UserNotFound(format!("user {}", id)))
    }

    #[instrument(skip(self))]
    pub async fn register(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<User, AuthError> {
        let hash = hash_password(&password).map_err(|err| AuthError::Internal(err.to_string()))?;
        let user = NewUser::new(email.to_lowercase(), username, hash);
        self.repo.create(user).await.map_err(AuthError::from)
    }

    #[instrument(skip(self))]
    pub async fn login(&self, email: &str, password: &str) -> Result<String, AuthError> {
        tracing::info!("user authenticated");
        let user = self
            .repo
            .find_by_email(&email.to_lowercase())
            .await
            .map_err(AuthError::from)?
            .ok_or_else(|| DomainError::Unauthorized)?;

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
