//! 消息置顶相关方法
//! 包含 pinChatMessage、unpinChatMessage、unpinAllChatMessages

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// pinChatMessage 参数
#[derive(Debug, Clone, Serialize)]
pub struct PinChatMessageParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 要置顶的消息 ID（必填）
    pub message_id: i32,
    /// 是否静默置顶（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

/// unpinChatMessage 参数
#[derive(Debug, Clone, Serialize)]
pub struct UnpinChatMessageParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 要取消置顶的消息 ID（选填，不传则取消最新置顶）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
}

/// unpinAllChatMessages 参数
#[derive(Debug, Clone, Serialize)]
pub struct UnpinAllChatMessagesParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
}

impl TelegramClient {
    /// 置顶消息
    /// 对应官方方法：pinChatMessage
    /// 需要 can_pin_messages 权限
    pub async fn pin_chat_message(&self, params: &PinChatMessageParams) -> TelegramResult<bool> {
        self.request("pinChatMessage", params).await
    }

    /// 取消置顶消息
    /// 对应官方方法：unpinChatMessage
    /// 需要 can_pin_messages 权限
    pub async fn unpin_chat_message(
        &self,
        params: &UnpinChatMessageParams,
    ) -> TelegramResult<bool> {
        self.request("unpinChatMessage", params).await
    }

    /// 取消所有置顶消息
    /// 对应官方方法：unpinAllChatMessages
    /// 需要 can_pin_messages 权限
    pub async fn unpin_all_chat_messages(
        &self,
        params: &UnpinAllChatMessagesParams,
    ) -> TelegramResult<bool> {
        self.request("unpinAllChatMessages", params).await
    }
}