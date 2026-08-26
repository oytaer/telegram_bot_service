//! HTTP 路由聚合
//! /api/admin/*  — 总后台（AdminAuth）
//! /api/agent/*  — 代理商（AgentAuth）
//! /api/tenant/* — 租户（TenantAuth）
//! /api/public/* — 登录等公开接口

use actix_web::web;

pub mod health;
pub mod components;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(health::configure)
            .configure(components::configure),
    );
}
