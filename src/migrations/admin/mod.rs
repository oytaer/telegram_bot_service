//! 平台库表（总后台 + 代理商）

pub mod admin_users;
pub mod agents;
pub mod agent_users;
pub mod tenants;
pub mod tenant_databases;
pub mod platform_configs;

pub async fn run_all(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    admin_users::migrate(pool).await?;
    agents::migrate(pool).await?;
    agent_users::migrate(pool).await?;
    tenants::migrate(pool).await?;
    tenant_databases::migrate(pool).await?;
    platform_configs::migrate(pool).await?;
    Ok(())
}
