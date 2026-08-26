//! 表：bot_flows — 机器人流程定义

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS bot_flows (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    bot_id          CHAR(36)        NOT NULL,
    name            VARCHAR(128)    NOT NULL,
    trigger_type    VARCHAR(64)     NOT NULL COMMENT 'command|message|callback|webhook|manual',
    trigger_config  JSON            NULL,
    status          TINYINT         NOT NULL DEFAULT 1,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_bot (bot_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
