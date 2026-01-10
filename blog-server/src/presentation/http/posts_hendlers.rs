use crate::application::blog_service::PostService;
use crate::data::post_repository::PostgresPostRepository;
use crate::domain::error::PostError;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::PostRequest;
use actix_web::{HttpResponse, Responder, Scope, delete, get, post, put, web};

use tracing;

pub fn scope() -> Scope {
    web::scope("/post")
        .service(create_post)
        .service(get_posts)
        .service(get_post)
        .service(update_post)
        .service(delete_post)
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
        Ok(post) => Ok(HttpResponse::Created().json(PostRequest {
            title: post.title,
            content: post.content,
        })),
        Err(e) => Err(e),
    }
}

#[get("")]
async fn get_posts(
    service: web::Data<PostService<PostgresPostRepository>>,
    user: AuthenticatedUser,
) -> Result<impl Responder, PostError> {
    let posts = service.get_posts().await?;
    tracing::info!(email = %user.email, "user logged in");
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{id}")]
async fn get_post(
    service: web::Data<PostService<PostgresPostRepository>>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<impl Responder, PostError> {
    let id = path.into_inner();
    let post = service.get_post(id.parse().unwrap()).await?;
    tracing::info!(email = %user.email, "user logged in");
    Ok(HttpResponse::Ok().json(post))
}

#[put("/{id}")]
async fn update_post(
    service: web::Data<PostService<PostgresPostRepository>>,
    user: AuthenticatedUser,
    path: web::Path<String>,
    payload: web::Json<PostRequest>,
) -> Result<impl Responder, PostError> {
    let id = path.into_inner();
    let post = service
        .update_post(
            id.parse().unwrap(),
            payload.title.clone(),
            payload.content.clone(),
            user,
        )
        .await?;
    Ok(HttpResponse::Ok().json(post))
}

#[delete("/{id}")]
async fn delete_post(
    service: web::Data<PostService<PostgresPostRepository>>,
    user: AuthenticatedUser,
    path: web::Path<String>,
) -> Result<impl Responder, PostError> {
    let id = path.into_inner();
    let post = service.get_post(id.parse().unwrap()).await?;
    if post.author_id != user.id {
        return Err(PostError::Unauthorized);
    }
    service.delete_post(id.parse().unwrap(), user).await?;
    Ok(HttpResponse::NoContent().finish())
}
