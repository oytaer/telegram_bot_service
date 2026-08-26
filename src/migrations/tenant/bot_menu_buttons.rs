//! 表：bot_menu_buttons — 底部菜单 / MenuButton / ReplyKeyboard 自定义

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS bot_menu_buttons (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    bot_id          CHAR(36)        NOT NULL,
    menu_type       VARCHAR(32)     NOT NULL COMMENT 'commands|web_app|default|reply_keyboard|inline',
    label           VARCHAR(128)    NOT NULL,
    config_json     JSON            NOT NULL COMMENT '按钮具体配置（url/web_app/callback等）',
    row_index       INT             NOT NULL DEFAULT 0,
    col_index       INT             NOT NULL DEFAULT 0,
    sort_order      INT             NOT NULL DEFAULT 0,
    chat_id         BIGINT          NULL COMMENT '私聊级菜单时使用',
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    INDEX idx_bot_type (bot_id, menu_type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
