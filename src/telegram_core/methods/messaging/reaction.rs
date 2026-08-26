//! 消息反应相关方法
//! 包含 setMessageReaction、deleteMessageReaction、deleteAllMessageReactions

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// 反应类型
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum ReactionType {
    #[serde(rename = "emoji")]
    Emoji { emoji: String },
    #[serde(rename = "custom_emoji")]
    CustomEmoji { custom_emoji_id: String },
    #[serde(rename = "paid")]
    Paid {},
}

/// setMessageReaction 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMessageReactionParams {
    pub chat_id: ChatId,
    pub message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reaction: Option<Vec<ReactionType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_big: Option<bool>,
}

/// deleteMessageReaction 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteMessageReactionParams {
    pub chat_id: ChatId,
    pub message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat_id: Option<i64>,
}

/// deleteAllMessageReactions 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteAllMessageReactionsParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat_id: Option<i64>,
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

    /// 删除消息上的单个反应
    /// 对应官方方法：deleteMessageReaction
    pub async fn delete_message_reaction(
        &self,
        params: &DeleteMessageReactionParams,
    ) -> TelegramResult<bool> {
        self.request("deleteMessageReaction", params).await
    }

    /// 删除用户/聊天在群组中的所有反应
    /// 对应官方方法：deleteAllMessageReactions
    pub async fn delete_all_message_reactions(
        &self,
        params: &DeleteAllMessageReactionsParams,
    ) -> TelegramResult<bool> {
        self.request("deleteAllMessageReactions", params).await
    }
}
