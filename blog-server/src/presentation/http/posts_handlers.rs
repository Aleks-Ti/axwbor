use crate::data::post_repository::PostgresPostRepository;
use crate::domain::error::PostError;
use crate::infrastructure::jwt::JwtKeys;
use crate::presentation::auth::AuthenticatedUser;
use crate::presentation::dto::{GetPostRequest, PostRequest};
use crate::{application::blog_service::PostService, presentation::middleware::JwtAuthMiddleware};
use actix_web::{HttpResponse, Responder, Scope, delete, get, post, put, web};

pub fn scope(keys: JwtKeys) -> Scope {
    web::scope("/post")
        // public
        .service(get_posts)
        .service(get_post)
        // protected
        .service(
            web::scope("")
                .wrap(JwtAuthMiddleware::new(keys))
                .service(create_post)
                .service(update_post)
                .service(delete_post),
        )
}

#[post("")]
async fn create_post(
    service: web::Data<PostService<PostgresPostRepository>>,
    user: AuthenticatedUser,
    payload: web::Json<PostRequest>,
) -> Result<impl Responder, PostError> {
    let new_post = service
        .create_post(payload.title.clone(), payload.content.clone(), user.id)
        .await?;
    Ok(HttpResponse::Created().json(new_post))
}

#[get("")]
async fn get_posts(
    service: web::Data<PostService<PostgresPostRepository>>,
    filter: web::Query<GetPostRequest>,
) -> Result<impl Responder, PostError> {
    let posts = service.get_posts(filter.limit, filter.offset).await?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/{id}")]
async fn get_post(
    service: web::Data<PostService<PostgresPostRepository>>,
    path: web::Path<String>,
) -> Result<impl Responder, PostError> {
    let id = path.into_inner();
    let post = service.get_post(id.parse().unwrap()).await?;
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
            user.id,
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
    service.delete_post(id.parse().unwrap(), user.id).await?;
    Ok(HttpResponse::NoContent().finish())
}
