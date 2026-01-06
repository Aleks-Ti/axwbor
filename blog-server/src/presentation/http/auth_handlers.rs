use crate::application::auth_service::AuthService;
use crate::data::user_repository::PostgresUserRepository;
use crate::domain::error::{AuthError};
use crate::presentation::dto::{LoginRequest, TokenResponse};
use actix_web::{HttpResponse, Responder, Scope, post, web};

use tracing;
pub fn scope() -> Scope {
    web::scope("auth").service(login)
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
