//! 表：bot_component_instances — 页面上的组件实例（可视化布局）

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS bot_component_instances (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    page_id         CHAR(36)        NOT NULL,
    bot_id          CHAR(36)        NOT NULL,
    component_id    VARCHAR(128)    NOT NULL COMMENT '注册表中的组件 ID',
    props_json      JSON            NOT NULL,
    pos_x           DOUBLE          NOT NULL DEFAULT 0,
    pos_y           DOUBLE          NOT NULL DEFAULT 0,
    width           DOUBLE          NULL,
    height          DOUBLE          NULL,
    z_index         INT             NOT NULL DEFAULT 0,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_page (page_id),
    INDEX idx_bot (bot_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
