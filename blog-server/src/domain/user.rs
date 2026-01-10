use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(id: i64, email: String, username: String, password_hash: String) -> Self {
        Self {
            id,
            email,
            username,
            password_hash,
            created_at: Utc::now(),
        }
    }
}

pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    pub fn new(email: String, username: String, password_hash: String) -> Self {
        Self {
            email,
            username,
            password_hash,
        }
    }
}
