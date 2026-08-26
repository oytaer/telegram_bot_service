//! 组件目录 API（租户画布拉取组件列表）

use actix_web::{get, web, HttpResponse};
use crate::controllers::components::ComponentRegistry;
use crate::middlewares::TenantAuth;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tenant/components")
            .wrap(TenantAuth)
            .service(list_components)
            .service(get_component),
    );
}

#[get("")]
async fn list_components() -> HttpResponse {
    let list = ComponentRegistry::global().list();
    HttpResponse::Ok().json(list)
}

#[get("/{id}")]
async fn get_component(path: web::Path<String>) -> HttpResponse {
    match ComponentRegistry::global().get(&path.into_inner()) {
        Some(c) => HttpResponse::Ok().json(c),
        None => HttpResponse::NotFound().json(serde_json::json!({ "error": "component not found" })),
    }
}
