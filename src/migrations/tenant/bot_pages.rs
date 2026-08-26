//! 表：bot_pages — 可视化页面（机器人/群组/频道等）

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS bot_pages (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    bot_id          CHAR(36)        NOT NULL,
    page_type       VARCHAR(32)     NOT NULL COMMENT 'bot|group|channel|custom',
    title           VARCHAR(128)    NOT NULL,
    layout_json     JSON            NOT NULL COMMENT '画布布局与组件位置',
    is_default      TINYINT         NOT NULL DEFAULT 0,
    sort_order      INT             NOT NULL DEFAULT 0,
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_bot (bot_id),
    INDEX idx_type (page_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
