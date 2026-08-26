//! 表：tenant_users — 租户侧用户

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS tenant_users (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    username        VARCHAR(64)     NOT NULL UNIQUE,
    password_hash   VARCHAR(255)    NOT NULL,
    display_name    VARCHAR(128)    NULL,
    role            VARCHAR(32)     NOT NULL DEFAULT 'member' COMMENT 'owner|admin|member',
    status          TINYINT         NOT NULL DEFAULT 1,
    last_login_at   DATETIME(3)     NULL,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
