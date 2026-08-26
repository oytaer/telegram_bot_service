//! 表：tenant_databases — 租户独立库连接信息（总后台可管理）

pub const TABLE: &str = "tenant_databases";

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS tenant_databases (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    tenant_id       CHAR(36)        NOT NULL UNIQUE,
    tenant_code     VARCHAR(64)     NOT NULL UNIQUE,
    db_host         VARCHAR(255)    NOT NULL,
    db_port         INT             NOT NULL DEFAULT 3306,
    db_name         VARCHAR(128)    NOT NULL,
    db_user         VARCHAR(128)    NOT NULL,
    db_password_enc TEXT            NOT NULL COMMENT '加密后的密码',
    is_external     TINYINT         NOT NULL DEFAULT 0 COMMENT '1=租户自带库独立部署',
    status          TINYINT         NOT NULL DEFAULT 1,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
