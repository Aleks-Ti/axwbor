use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Структура, представляющая пользователя в системе.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

/// Структура, представляющая нового пользователя для регистрации.
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

impl NewUser {
    /// Создаёт новый экземпляр NewUser.
    pub fn new(email: String, username: String, password_hash: String) -> Self {
        Self {
            email,
            username,
            password_hash,
        }
    }
}
