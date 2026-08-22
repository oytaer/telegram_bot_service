//! 论坛话题相关类型定义

use serde::{Deserialize, Serialize};

/// 论坛话题（官方 ForumTopic）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    /// 话题消息线程 ID
    pub message_thread_id: i32,
    /// 话题名称
    pub name: String,
    /// 图标颜色
    pub icon_color: i32,
    /// 自定义 emoji 图标 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}