//! 消息删除相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// deleteMessage 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteMessageParams {
    /// 聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息 ID（必填）
    pub message_id: i32,
}

/// deleteMessages 参数（批量删除）
#[derive(Debug, Clone, Serialize)]
pub struct DeleteMessagesParams {
    /// 聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息 ID 列表（必填，1-100个）
    pub message_ids: Vec<i32>,
}

impl TelegramClient {
    /// 删除单条消息
    /// 对应官方方法：deleteMessage
    /// 删除他人消息需要 can_delete_messages 权限
    pub async fn delete_message(&self, params: &DeleteMessageParams) -> TelegramResult<bool> {
        self.request("deleteMessage", params).await
    }

    /// 批量删除消息
    /// 对应官方方法：deleteMessages
    pub async fn delete_messages(&self, params: &DeleteMessagesParams) -> TelegramResult<bool> {
        self.request("deleteMessages", params).await
    }
}