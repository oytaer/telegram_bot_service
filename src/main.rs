mod config;
mod db;
mod auth;
mod migrations;
mod utils;
mod routers;
mod middlewares;
mod controllers;

// telegram_core 为完整的 Telegram Bot API 封装库
#[allow(dead_code)]
mod telegram_core;

use actix_web::{web, App, HttpServer};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("info".parse()?))
        .init();

    let cfg = config::AppConfig::from_env()?;
    let platform_pool = db::platform::create_platform_pool(&cfg.platform_database_url).await?;
    let tenant_manager = db::tenant::TenantDbManager::new(cfg.clone());

    let app_state = web::Data::new(AppState {
        config: cfg.clone(),
        platform_pool,
        tenant_manager,
    });

    let bind = format!("{}:{}", cfg.host, cfg.port);
    tracing::info!("listening on {bind}");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routers::configure)
    })
    .bind(&bind)?
    .run()
    .await?;

    Ok(())
}

/// 全局应用状态
#[derive(Clone)]
pub struct AppState {
    pub config: config::AppConfig,
    pub platform_pool: sqlx::MySqlPool,
    pub tenant_manager: db::tenant::TenantDbManager,
}
