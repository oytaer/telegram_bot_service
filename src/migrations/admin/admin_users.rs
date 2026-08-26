//! 表：admin_users — 总后台管理员

pub const TABLE: &str = "admin_users";

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS admin_users (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    username        VARCHAR(64)     NOT NULL UNIQUE,
    password_hash   VARCHAR(255)    NOT NULL,
    display_name    VARCHAR(128)    NULL,
    status          TINYINT         NOT NULL DEFAULT 1 COMMENT '1=active 0=disabled',
    last_login_at   DATETIME(3)     NULL,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
