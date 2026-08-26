//! 租户库表（每个租户独立数据库）

pub mod tenant_users;
pub mod tenant_bots;
pub mod bot_pages;
pub mod bot_flows;
pub mod bot_nodes;
pub mod bot_edges;
pub mod bot_commands;
pub mod bot_menu_buttons;
pub mod bot_component_instances;

pub async fn run_all(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    tenant_users::migrate(pool).await?;
    tenant_bots::migrate(pool).await?;
    bot_pages::migrate(pool).await?;
    bot_flows::migrate(pool).await?;
    bot_nodes::migrate(pool).await?;
    bot_edges::migrate(pool).await?;
    bot_commands::migrate(pool).await?;
    bot_menu_buttons::migrate(pool).await?;
    bot_component_instances::migrate(pool).await?;
    Ok(())
}
