//! 表：tenant_bots — 租户下的多个 Telegram Bot（相互隔离）

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS tenant_bots (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    name            VARCHAR(128)    NOT NULL,
    username        VARCHAR(64)     NULL COMMENT 'Bot @username',
    token_enc       TEXT            NOT NULL COMMENT '加密存储的 Bot Token',
    telegram_id     BIGINT          NULL,
    webhook_url     VARCHAR(512)    NULL,
    status          TINYINT         NOT NULL DEFAULT 1 COMMENT '1=active 0=disabled',
    description     VARCHAR(512)    NULL,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
