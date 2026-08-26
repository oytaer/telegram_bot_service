//! 平台库连接（总后台 + 代理商）

use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

pub async fn create_platform_pool(database_url: &str) -> anyhow::Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await?;
    Ok(pool)
}
