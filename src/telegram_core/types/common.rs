//! 通用基础类型定义

use serde::{Deserialize, Serialize};

/// Telegram 中的聊天 ID（支持数字 ID 或 @username）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)] // 支持数字或字符串两种形式
pub enum ChatId {
    /// 数字形式的聊天 ID
    Id(i64),
    /// 用户名形式（例如 @channelusername）
    Username(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        ChatId::Id(id)
    }
}

impl From<String> for ChatId {
    fn from(username: String) -> Self {
        ChatId::Username(username)
    }
}

impl From<&str> for ChatId {
    fn from(username: &str) -> Self {
        ChatId::Username(username.to_string())
    }
}

/// 消息实体类型（加粗、斜体、链接等）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEntity {
    /// 实体类型（bold, italic, code, text_link 等）
    #[serde(rename = "type")]
    pub type_field: String,
    /// 起始位置（UTF-16 码元）
    pub offset: i32,
    /// 长度（UTF-16 码元）
    pub length: i32,
    /// 可选：链接地址（当 type 为 text_link 时）
    pub url: Option<String>,
    /// 可选：用户信息（当 type 为 text_mention 时）
    pub user: Option<super::user::User>,
    /// 可选：编程语言（当 type 为 pre 时）
    pub language: Option<String>,
    /// 可选：自定义 emoji ID
    pub custom_emoji_id: Option<String>,
}