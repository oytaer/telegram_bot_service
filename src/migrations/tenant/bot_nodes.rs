//! 表：bot_nodes — 流程节点（对应拖拽组件实例）

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS bot_nodes (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    flow_id         CHAR(36)        NOT NULL,
    bot_id          CHAR(36)        NOT NULL,
    component_id    VARCHAR(128)    NOT NULL COMMENT '如 telegram.send_message',
    title           VARCHAR(128)    NULL,
    config_json     JSON            NOT NULL COMMENT '组件入参配置',
    pos_x           DOUBLE          NOT NULL DEFAULT 0,
    pos_y           DOUBLE          NOT NULL DEFAULT 0,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_flow (flow_id),
    INDEX idx_bot (bot_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
