use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};

/// Структура, содержащая секретный ключ для подписи JWT токенов.
#[derive(Clone)]
pub struct JwtKeys {
    secret: String,
}

/// Методы для генерации и проверки JWT токенов, а также для хеширования и проверки паролей.
impl JwtKeys {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
    /// Генерирует JWT токен для указанного user_id.
    pub fn generate_token(&self, user_id: i64) -> Result<String, jsonwebtoken::errors::Error> {
        let claims = Claims {
            sub: user_id.to_string(),
            exp: chrono::Utc::now()
                .checked_add_signed(chrono::Duration::hours(1))
                .unwrap()
                .timestamp() as usize,
            iat: chrono::Utc::now().timestamp() as usize,
        };
        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
    }
    /// Проверяет валидность предоставленного JWT токена и возвращает его претензии (claims).
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )?;
        Ok(data.claims)
    }
}

/// Структура, представляющая JWT претензии (claims).
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

/// Хеширует предоставленный пароль с использованием Argon2.
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    Ok(hash)
}

/// Проверяет, соответствует ли предоставленный пароль хешу.
pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed = PasswordHash::new(hash)?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(password.as_bytes(), &parsed).is_ok())
}
