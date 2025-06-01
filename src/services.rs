use actix_web::web::scope;
use actix_web::{HttpResponse, Responder, get, web};
use serde_json::json;

#[get("/health")]
async fn health() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running...";

    HttpResponse::Ok().json(json!({"status": "ok", "message": MESSAGE}))
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = scope("/api").service(health);

    conf.service(scope);
}
