//! 用户相关类型

use serde::{Deserialize, Serialize};

/// Telegram 用户对象（官方 User）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户唯一 ID
    pub id: i64,
    /// 是否是机器人
    pub is_bot: bool,
    /// 名
    pub first_name: String,
    /// 姓（可选）
    pub last_name: Option<String>,
    /// 用户名（可选）
    pub username: Option<String>,
    /// 语言代码（可选，IETF 语言标签）
    pub language_code: Option<String>,
    /// 是否是 Premium 用户（可选）
    pub is_premium: Option<bool>,
    /// 是否添加了附件菜单（可选）
    pub added_to_attachment_menu: Option<bool>,
    /// 是否可以加入群组（仅 Bot 返回）
    pub can_join_groups: Option<bool>,
    /// 是否可以读取所有群消息（仅 Bot 返回）
    pub can_read_all_group_messages: Option<bool>,
    /// 是否支持内联查询（仅 Bot 返回）
    pub supports_inline_queries: Option<bool>,
    /// 是否支持 Guest 查询（新版本）
    pub supports_guest_queries: Option<bool>,
}