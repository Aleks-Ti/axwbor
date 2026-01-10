use std::sync::Arc;

use crate::data::post_repository::PostRepository;
use crate::domain::post::NewPost;
use crate::domain::{error::PostError, post::Post};
use crate::presentation::auth::AuthenticatedUser;

#[derive(Clone)]
pub struct PostService<R: PostRepository + 'static> {
    repo: Arc<R>,
}

impl<R> PostService<R>
where
    R: PostRepository + 'static,
{
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    pub async fn create_post(
        &self,
        title: String,
        content: String,
        author_id: i64,
    ) -> Result<Post, PostError> {
        let post = NewPost::new(title, content, author_id);
        let new_post = self.repo.create(post).await.map_err(PostError::from);
        println!("{:?}", new_post);
        new_post
    }

    pub async fn get_posts(&self) -> Result<Vec<Post>, PostError> {
        self.repo
            .find_all()
            .await
            .map_err(PostError::from)?
            .ok_or_else(|| PostError::NotFound("posts not found".into()))
    }

    pub async fn get_post(&self, id: i64) -> Result<Post, PostError> {
        self.repo
            .find_by_id(id)
            .await
            .map_err(PostError::from)?
            .ok_or_else(|| PostError::NotFound(format!("post {} not found", id)))
    }

    pub async fn update_post(
        &self,
        id: i64,
        title: String,
        content: String,
        current_user: AuthenticatedUser,
    ) -> Result<Post, PostError> {
        let post = self.repo.find_by_id(id).await.map_err(PostError::from)?;
        if post.is_none() {
            return Err(PostError::NotFound(format!("post {} not found", id)));
        }
        if post.unwrap().author_id != current_user.id {
            return Err(PostError::Unauthorized);
        }
        let post = NewPost::new(title, content, current_user.id);
        self.repo
            .update(id, post)
            .await
            .map_err(PostError::from)?
            .ok_or_else(|| PostError::NotFound(format!("post {} not found", id)))
    }

    pub async fn delete_post(
        &self,
        id: i64,
        current_user: AuthenticatedUser,
    ) -> Result<(), PostError> {
        let post = self.repo.find_by_id(id).await.map_err(PostError::from)?;
        if post.is_none() {
            return Err(PostError::NotFound(format!("post {} not found", id)));
        }
        if post.unwrap().author_id != current_user.id {
            return Err(PostError::Unauthorized);
        }
        let _ = self.repo.delete(id).await.map_err(PostError::from);
        Ok(())
    }
}
