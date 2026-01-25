use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Структура, представляющая пост в блоге.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
}

/// Структура, представляющая новый пост для создания.
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
}

/// Методы для создания нового поста.
impl NewPost {
    pub fn new(title: String, content: String, author_id: i64) -> Self {
        Self {
            title,
            content,
            author_id,
            created_at: Utc::now(),
        }
    }
}
