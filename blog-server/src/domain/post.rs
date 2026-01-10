use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(id: i64, title: String, content: String, author_id: i64) -> Self {
        Self {
            id,
            title,
            content,
            author_id,
            created_at: Utc::now(),
        }
    }
}

pub struct NewPost {
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: DateTime<Utc>,
}

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
