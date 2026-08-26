//! 表：platform_configs — 平台级配置

pub const TABLE: &str = "platform_configs";

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS platform_configs (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    config_key      VARCHAR(128)    NOT NULL UNIQUE,
    config_value    JSON            NOT NULL,
    description     VARCHAR(255)    NULL,
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
