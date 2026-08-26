//! 表：agents — 代理商主体

pub const TABLE: &str = "agents";

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS agents (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    code            VARCHAR(64)     NOT NULL UNIQUE COMMENT '代理商编码',
    name            VARCHAR(128)    NOT NULL,
    contact_email   VARCHAR(255)    NULL,
    contact_phone   VARCHAR(32)     NULL,
    status          TINYINT         NOT NULL DEFAULT 1,
    parent_agent_id CHAR(36)        NULL COMMENT '上级代理，可空',
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_parent (parent_agent_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
