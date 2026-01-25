use std::sync::Arc;

use crate::data::post_repository::PostRepository;
use crate::domain::post::NewPost;
use crate::domain::{error::PostError, post::Post};

#[derive(Clone)]
pub struct PostService<R: PostRepository + 'static> {
    repo: Arc<R>,
}

impl<R> PostService<R>
where
    R: PostRepository + 'static,
{   /// Создаёт новый экземпляр PostService.
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    /// Создание нового поста.
    pub async fn create_post(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, PostError> {
        let post = NewPost::new(title, content, author_id);
        self.repo.create(post).await
    }

    /// Получение списка постов с пагинацией.
    pub async fn get_posts(&self, limit: i32, offset: i32) -> Result<Vec<Post>, PostError> {
        self.repo
            .find_all(limit, offset)
            .await?
            .ok_or_else(|| PostError::PostNotFound("posts not found".into()))
    }

    /// Получение поста по ID.
    pub async fn get_post(&self, id: i64) -> Result<Post, PostError> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| PostError::PostNotFound(format!("post {} not found", id)))
    }

    /// Обновление поста.
    pub async fn update_post(
        &self,
        id: i64,
        title: Option<String>,
        content: Option<String>,
        current_user_id: i64,
    ) -> Result<Post, PostError> {
        let post = self.repo.find_by_id(id).await?;
        if post.is_none() {
            return Err(PostError::PostNotFound(format!("post {} not found", id)));
        }
        if post.unwrap().author_id != current_user_id {
            return Err(PostError::Forbidden);
        }
        self.repo
            .update(id, title, content)
            .await?
            .ok_or_else(|| PostError::PostNotFound(format!("post {} not found", id)))
    }

    /// Удаление поста.
    pub async fn delete_post(&self, id: i64, current_user_id: i64) -> Result<(), PostError> {
        let post = self.repo.find_by_id(id).await?;
        if post.is_none() {
            return Err(PostError::PostNotFound(format!("post {} not found", id)));
        }
        if post.unwrap().author_id != current_user_id {
            return Err(PostError::Forbidden);
        }
        self.repo.delete(id).await?;
        Ok(())
    }
}
