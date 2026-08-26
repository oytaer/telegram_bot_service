//! 表：bot_edges — 流程连线

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS bot_edges (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    flow_id         CHAR(36)        NOT NULL,
    source_node_id  CHAR(36)        NOT NULL,
    target_node_id  CHAR(36)        NOT NULL,
    source_handle   VARCHAR(64)     NULL,
    target_handle   VARCHAR(64)     NULL,
    condition_json  JSON            NULL COMMENT '条件分支配置',
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    INDEX idx_flow (flow_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
