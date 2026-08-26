//! 表：tenants — 租户注册（平台库）

pub const TABLE: &str = "tenants";

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS tenants (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    code            VARCHAR(64)     NOT NULL UNIQUE COMMENT '租户编码，用于库名/连接模板',
    name            VARCHAR(128)    NOT NULL,
    agent_id        CHAR(36)        NULL COMMENT '所属代理商，可空=直营',
    status          TINYINT         NOT NULL DEFAULT 1 COMMENT '1=active 0=disabled 2=suspended',
    plan_code       VARCHAR(64)     NULL COMMENT '套餐编码',
    max_bots        INT             NOT NULL DEFAULT 5,
    deploy_mode     VARCHAR(16)     NOT NULL DEFAULT 'saas' COMMENT 'saas|standalone',
    expires_at      DATETIME(3)     NULL,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_agent (agent_id),
    INDEX idx_status (status)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
