//! 数据库迁移 / 表定义
//! admin：平台库（总后台 + 代理商）
//! tenant：租户独立库
//! 约定：一表一文件，文件内包含建表 SQL

pub mod admin;
pub mod tenant;

/// 在平台库执行全部 admin 迁移
pub async fn migrate_platform(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    admin::run_all(pool).await
}

/// 在指定租户库执行全部 tenant 迁移
pub async fn migrate_tenant(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    tenant::run_all(pool).await
}
