//! 内联与回调相关类型定义

use serde::{Deserialize, Serialize};
use super::user::User;
use super::message::Message;
use super::chat::Location;

/// 内联查询（官方 InlineQuery）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineQuery {
    /// 查询唯一 ID
    pub id: String,
    /// 发起查询的用户
    pub from: User,
    /// 查询文本
    pub query: String,
    /// 偏移量
    pub offset: String,
    /// 聊天类型（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,
    /// 位置（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

/// 选择的内联结果（官方 ChosenInlineResult）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// 结果 ID
    pub result_id: String,
    /// 选择结果的用户
    pub from: User,
    /// 位置（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// 内联消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// 查询文本
    pub query: String,
}

/// 回调查询（官方 CallbackQuery）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackQuery {
    /// 查询唯一 ID
    pub id: String,
    /// 发起回调的用户
    pub from: User,
    /// 消息（选填，可能是普通消息或内联消息）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,
    /// 内联消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// 聊天实例
    pub chat_instance: String,
    /// 回调数据（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    /// 游戏短名称（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_short_name: Option<String>,
}