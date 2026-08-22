//! 消息反应相关方法
//! 包含 setMessageReaction

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::update::ReactionType;

/// setMessageReaction 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMessageReactionParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,

    /// 目标消息 ID（必填）
    pub message_id: i32,

    /// 反应列表（选填，传空数组可移除所有反应）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<ReactionType>>,

    /// 是否为大气反应（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

impl TelegramClient {
    /// 设置消息反应
    /// 对应官方方法：setMessageReaction
    pub async fn set_message_reaction(
        &self,
        params: &SetMessageReactionParams,
    ) -> TelegramResult<bool> {
        self.request("setMessageReaction", params).await
    }
}