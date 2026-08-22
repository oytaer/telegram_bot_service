//! 消息复制与转发相关方法
//! 包含 copyMessage、copyMessages、forwardMessage、forwardMessages

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::{ChatId, MessageEntity};
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::keyboard::ReplyMarkup;
use super::send::text::ReplyParameters;

/// copyMessage 参数
#[derive(Debug, Clone, Serialize)]
pub struct CopyMessageParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息线程/话题 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 来源聊天 ID（必填）
    pub from_chat_id: ChatId,
    /// 要复制的消息 ID（必填）
    pub message_id: i32,
    /// 新标题（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// 解析模式（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// 标题实体（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    /// 是否在媒体上方显示标题（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    /// 是否禁用通知（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// 是否保护内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// 是否允许付费广播（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// 回复参数（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// 回复标记（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// copyMessages 参数（批量复制）
#[derive(Debug, Clone, Serialize)]
pub struct CopyMessagesParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息线程/话题 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 来源聊天 ID（必填）
    pub from_chat_id: ChatId,
    /// 要复制的消息 ID 列表（必填，1-100个）
    pub message_ids: Vec<i32>,
    /// 是否禁用通知（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// 是否保护内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// 是否移除标题（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_caption: Option<bool>,
}

/// forwardMessage 参数
#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessageParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息线程/话题 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 来源聊天 ID（必填）
    pub from_chat_id: ChatId,
    /// 要转发的消息 ID（必填）
    pub message_id: i32,
    /// 是否禁用通知（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// 是否保护内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

/// forwardMessages 参数（批量转发）
#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessagesParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息线程/话题 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 来源聊天 ID（必填）
    pub from_chat_id: ChatId,
    /// 要转发的消息 ID 列表（必填，1-100个）
    pub message_ids: Vec<i32>,
    /// 是否禁用通知（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// 是否保护内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

/// 复制消息返回的结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageId {
    pub message_id: i32,
}

impl TelegramClient {
    /// 复制消息（不显示转发来源）
    /// 对应官方方法：copyMessage
    pub async fn copy_message(&self, params: &CopyMessageParams) -> TelegramResult<MessageId> {
        self.request("copyMessage", params).await
    }

    /// 批量复制消息
    /// 对应官方方法：copyMessages
    pub async fn copy_messages(
        &self,
        params: &CopyMessagesParams,
    ) -> TelegramResult<Vec<MessageId>> {
        self.request("copyMessages", params).await
    }

    /// 转发消息
    /// 对应官方方法：forwardMessage
    pub async fn forward_message(&self, params: &ForwardMessageParams) -> TelegramResult<Message> {
        self.request("forwardMessage", params).await
    }

    /// 批量转发消息
    /// 对应官方方法：forwardMessages
    pub async fn forward_messages(
        &self,
        params: &ForwardMessagesParams,
    ) -> TelegramResult<Vec<MessageId>> {
        self.request("forwardMessages", params).await
    }
}