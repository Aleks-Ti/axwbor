use crate::application::blog_service::PostService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::error::{AuthError, PostError};
use crate::domain::post::Post;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::{LoginRequest, PostRequest};
use actix_web::{HttpResponse, Responder, Scope, get, post, web};

use tracing;

pub fn scope() -> Scope {
    web::scope("/post").service(create_post)
}

#[post("")]
async fn create_post(
    service: web::Data<PostService<PostgresPostRepository>>,
    user: AuthenticatedUser,
    payload: web::Json<PostRequest>,
) -> Result<impl Responder, PostError> {
    println!("{:?}", user);
    let new_post = service
        .create_post(payload.title.clone(), payload.content.clone(), user.id)
        .await;
    match new_post {
        Ok(post) => Ok(HttpResponse::Created().json(PostRequest { title: post.title, content: post.content })),
        Err(e) => Err(e),
    }
}

// #[get("")]
// async fn get_posts(
//     service: web::Data<PostService<PostgresPostRepository>>,
//     payload: web::Json<LoginRequest>,
// ) -> Result<impl Responder, AuthError> {
//     let jwt = service.login(&payload.username, &payload.password).await?;
//     tracing::info!(email = %payload.username, "user logged in");
//     Ok(HttpResponse::Ok().json(TokenResponse { access_token: jwt }))
// }

// #[get("/{id}")]
// async fn get_post(
//     service: web::Data<AuthService<PostgresUserRepository>>,
//     path: web::Path<String>,
// ) -> Result<impl Responder, AuthError> {
//     let id = path.into_inner();
//     // let jwt = service.login(&payload.username, &payload.password).await?;
//     // tracing::info!(email = %payload.username, "user logged in");
//     Ok(HttpResponse::Ok().json(TokenResponse { access_token: jwt }))
// }
