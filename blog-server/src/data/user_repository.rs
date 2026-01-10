use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tracing;

use crate::domain::{error::AuthError, user::NewUser, user::User};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: NewUser) -> Result<User, AuthError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, AuthError>;
}

#[derive(Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, user: NewUser) -> Result<User, AuthError> {
        let row = sqlx::query(
            r#"
            INSERT INTO users (email, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, email, username, password_hash, created_at
            "#,
        )
        .bind(&user.email)
        .bind(&user.username)
        .bind(&user.password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to create user: {}", e);
            if e.as_database_error()
                .and_then(|db| db.constraint())
                .map(|c| c.contains("users_email"))
                == Some(true)
            {
                AuthError::Validation("email already registered".into())
            } else {
                AuthError::Internal(format!("database error: {}", e))
            }
        })?;
        let user_dto = User {
            id: row.get("id"),
            email: user.email,
            username: user.username,
            password_hash: user.password_hash,
            created_at: chrono::Utc::now(),
        };
        tracing::info!(user_id = %user_dto.id, email = %user_dto.email, "user created");
        Ok(user_dto)
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AuthError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, created_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to find user by email {}: {}", email, e);
            AuthError::Internal(format!("database error: {}", e))
        })?;

        Ok(row.map(|row| User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        }))
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, AuthError> {
        let row = sqlx::query(
            r#"
            SELECT id, email, username, password_hash, created_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to find user by id {}: {}", id, e);
            AuthError::Internal(format!("database error: {}", e))
        })?;

        Ok(row.map(|row| User {
            id: row.get("id"),
            email: row.get("email"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            created_at: row.get("created_at"),
        }))
    }
}
