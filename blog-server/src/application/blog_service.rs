use std::sync::Arc;

use tracing::instrument;

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

    // pub async fn get_posts(&self, id: i64) -> Result<Post, PostError> {
    //     self.repo
    //         .find_by_id(id)
    //         .await
    //         .map_err(PostError::from)?
    //         .ok_or_else(|| PostError::NotFound(format!("user {}", id)))
    // }
}
