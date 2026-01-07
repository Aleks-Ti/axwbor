use crate::presentation::dto::HealthResponse;
use actix_web::{HttpResponse, Responder, Scope, web};
use chrono::Utc;

pub fn scope() -> Scope {
    web::scope("/help").route("/health", web::get().to(health))
}

async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse {
        status: "ok",
        timestamp: Utc::now(),
    })
}
