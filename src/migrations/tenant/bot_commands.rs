//! 表：bot_commands — 机器人命令菜单（可拖拽排序）

pub const CREATE_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS bot_commands (
    id              CHAR(36)        NOT NULL PRIMARY KEY,
    bot_id          CHAR(36)        NOT NULL,
    command         VARCHAR(32)     NOT NULL COMMENT '不含斜杠，如 start',
    description     VARCHAR(256)    NOT NULL,
    scope           VARCHAR(64)     NOT NULL DEFAULT 'default' COMMENT 'default|all_private|all_group|chat|...',
    language_code   VARCHAR(16)     NOT NULL DEFAULT '',
    sort_order      INT             NOT NULL DEFAULT 0,
    is_ephemeral    TINYINT         NOT NULL DEFAULT 0,
    flow_id         CHAR(36)        NULL COMMENT '绑定的流程',
    created_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
    updated_at      DATETIME(3)     NOT NULL DEFAULT CURRENT_TIMESTAMP(3) ON UPDATE CURRENT_TIMESTAMP(3),
    UNIQUE KEY uk_bot_cmd_scope_lang (bot_id, command, scope, language_code),
    INDEX idx_bot (bot_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
"#;

pub async fn migrate(pool: &sqlx::MySqlPool) -> anyhow::Result<()> {
    sqlx::query(CREATE_SQL).execute(pool).await?;
    Ok(())
}
