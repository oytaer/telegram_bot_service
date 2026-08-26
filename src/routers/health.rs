use actix_web::{get, web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(health);
}

#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}
