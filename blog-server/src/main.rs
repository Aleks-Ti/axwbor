include!(concat!(env!("OUT_DIR"), "/blog.rs"));
mod application;
mod data;
mod domain;
mod infrastructure;
mod presentation;

use actix_cors::Cors;
use actix_web::middleware::{DefaultHeaders, Logger};
use actix_web::{App, HttpServer, web};
use application::auth_service::AuthService;
use application::blog_service::PostService;
use data::post_repository::PostgresPostRepository;
use data::user_repository::PostgresUserRepository;
use infrastructure::config::AppConfig;
use infrastructure::database::{create_pool, run_migrations};
use infrastructure::jwt::JwtKeys;
use infrastructure::logging::init_logging;
use presentation::http::{auth_handlers, help_handlers, posts_hendlers};
use presentation::middleware::{JwtAuthMiddleware, RequestIdMiddleware, TimingMiddleware};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_logging();

    let config = AppConfig::from_env().expect("invalid configuration");
    let pool = create_pool(&config.database_url)
        .await
        .expect("failed to connect to database");

    run_migrations(&pool)
        .await
        .expect("failed to run migrations");

    let user_repo = Arc::new(PostgresUserRepository::new(pool.clone()));
    let post_repo = Arc::new(PostgresPostRepository::new(pool.clone()));

    let auth_service = Arc::new(AuthService::new(
        Arc::clone(&user_repo),
        JwtKeys::new(config.jwt_secret.clone()),
    ));
    let post_service = Arc::new(PostService::new(post_repo));

    // === HTTP-сервер ===
    let http_config = Arc::new(config.clone());
    let http_post_service = post_service.clone();
    let http_auth_service = auth_service.clone();
    let grpc_post_service = post_service.clone();
    let grpc_auth_service = auth_service.clone();

    let http_config_clone = Arc::clone(&http_config);
    let http_handle = HttpServer::new(move || {
        let cors = build_cors(&http_config_clone);
        App::new()
            .wrap(Logger::default())
            .wrap(RequestIdMiddleware)
            .wrap(TimingMiddleware)
            .wrap(DefaultHeaders::new().add(("X-Content-Type-Options", "nosniff")))
            .wrap(cors)
            .app_data(web::Data::from(http_auth_service.clone()))
            .app_data(web::Data::from(http_post_service.clone()))
            .service(
                web::scope("/api")
                    .service(help_handlers::scope())
                    .service(auth_handlers::scope())
                    .service(
                        posts_hendlers::scope()
                            .wrap(JwtAuthMiddleware::new(http_auth_service.keys().clone())),
                    ),
            )
    })
    .bind((http_config.host.as_str(), http_config.port))?
    .run();

    fn build_cors(config: &AppConfig) -> Cors {
        let mut cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(3600);

        for origin in &config.cors_origins {
            cors = cors.allowed_origin(origin);
        }
        cors
    }
    // === gRPC-сервер ===
    let grpc_post_service = grpc_post_service.clone();
    let grpc_addr = format!("{}:{}", config.host, config.grpc_port);

    let grpc_handle = tokio::spawn(async move {
        let grpc_impl = presentation::grpc::PostGrpcService::new(grpc_post_service);
        let tonic_svc = crate::post_service_server::PostServiceServer::new(grpc_impl);
        tonic::transport::Server::builder()
            .add_service(tonic_svc)
            .serve(grpc_addr.parse().unwrap())
            .await
    });

    tokio::select! {
        _ = http_handle => {},
        _ = grpc_handle => {},
    }

    Ok(())
}
