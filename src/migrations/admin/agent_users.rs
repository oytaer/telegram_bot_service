//! 表：agent_users — 代理商登录账号

pub const TABLE: &str = "agent_users";

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS agent_users (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    agent_id        CHAR(36)        NOT NULL,
    username        VARCHAR(64)     NOT NULL,
    password_hash   VARCHAR(255)    NOT NULL,
    display_name    VARCHAR(128)    NULL,
    status          TINYINT         NOT NULL DEFAULT 1,
    last_login_at   DATETIME(3)     NULL,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    UNIQUE KEY uk_agent_username (agent_id, username),
    INDEX idx_agent (agent_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
