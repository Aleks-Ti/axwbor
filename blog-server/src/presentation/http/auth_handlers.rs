use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::error::AuthError;
use crate::presentation::dto::{LoginRequest, RegisterRequest, TokenResponse};
use actix_web::{HttpResponse, Responder, Scope, post, web};

use tracing;
pub fn scope() -> Scope {
    web::scope("/auth").service(login).service(register)
}

#[post("/login")]
async fn login(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<LoginRequest>,
) -> Result<impl Responder, AuthError> {
    let jwt = service.login(&payload.username, &payload.password).await?;
    tracing::info!(email = %payload.username, "user logged in");
    Ok(HttpResponse::Ok().json(TokenResponse { access_token: jwt }))
}

#[post("/register")]
async fn register(
    service: web::Data<AuthService<PostgresUserRepository>>,
    payload: web::Json<RegisterRequest>,
) -> Result<impl Responder, AuthError> {
    let user = service
        .register(payload.email.clone(), payload.username.clone(), payload.password.clone())
        .await?;

    tracing::info!(user_id = %user.id, email = %user.email, "user registered");

    Ok(HttpResponse::Created().json(serde_json::json!({
        "user_id": user.id,
        "email": user.email
    })))
}
