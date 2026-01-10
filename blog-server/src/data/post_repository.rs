use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tracing;

use crate::domain::{error::PostError, post::NewPost, post::Post};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create(&self, post: NewPost) -> Result<Post, PostError>;
    async fn find_by_id(&self, id: i64) -> Result<Option<Post>, PostError>;
    async fn find_all(&self) -> Result<Option<Post>, PostError>;
    async fn update(&self, id: i64, post: NewPost) -> Result<Option<Post>, PostError>;
    async fn delete(&self, id: i64) -> Result<Option<Post>, PostError>;
}

#[derive(Clone)]
pub struct PostgresPostRepository {
    pool: PgPool,
}

impl PostgresPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for PostgresPostRepository {
    async fn create(&self, post: NewPost) -> Result<Post, PostError> {
        let row = sqlx::query(
            r#"
            INSERT INTO posts (title, content, author_id, created_at)
            VALUES ($1, $2, $3, $4)
            RETURNING id, title, content, author_id, created_at
            "#,
        )
        .bind(&post.title)
        .bind(&post.content)
        .bind(&post.author_id)
        .bind(&post.created_at)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            tracing::error!("failed to create post: {}", e);
            if e.as_database_error()
                .and_then(|db| db.constraint())
                .map(|c| c.contains("users_email"))
                == Some(true)
            {
                PostError::Validation("email already registered".into())
            } else {
                PostError::Internal(format!("database error: {}", e))
            }
        })?;
        let post_dto = Post {
            id: row.get("id"),
            title: row.get("title"),
            content: row.get("content"),
            author_id: row.get("author_id"),
            created_at: chrono::Utc::now(),
        };
        tracing::info!(post_id = %post_dto.id, title = %post_dto.title, "post created");
        Ok(post_dto)
    }

    async fn find_by_id(&self, _id: i64) -> Result<Option<Post>, PostError> {
        todo!("implement find_by_id")
    }

    async fn find_all(&self) -> Result<Option<Post>, PostError> {
        todo!("implement find_all")
    }

    async fn update(&self, _id: i64, _post: NewPost) -> Result<Option<Post>, PostError> {
        todo!("implement update")
    }

    async fn delete(&self, _id: i64) -> Result<Option<Post>, PostError> {
        todo!("implement delete")
    }
}
