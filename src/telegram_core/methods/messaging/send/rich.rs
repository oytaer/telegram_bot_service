//! 富文本消息发送方法
//! 包含 sendRichMessage、sendRichMessageDraft

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::keyboard::ReplyMarkup;
use crate::telegram_core::types::rich_message::InputRichMessage;
use super::text::{ReplyParameters, SuggestedPostParameters};

/// sendRichMessage 参数
#[derive(Debug, Clone, Serialize)]
pub struct SendRichMessageParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,
    /// 富文本内容（必填）
    pub rich_message: InputRichMessage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// sendRichMessageDraft 参数（流式草稿）
#[derive(Debug, Clone, Serialize)]
pub struct SendRichMessageDraftParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 草稿 ID（用于后续更新）
    pub draft_id: i64,
    /// 富文本内容
    pub rich_message: InputRichMessage,
}

impl TelegramClient {
    /// 发送富文本消息
    /// 对应官方方法：sendRichMessage
    pub async fn send_rich_message(
        &self,
        params: &SendRichMessageParams,
    ) -> TelegramResult<Message> {
        self.request("sendRichMessage", params).await
    }

    /// 发送富文本草稿（流式）
    /// 对应官方方法：sendRichMessageDraft
    pub async fn send_rich_message_draft(
        &self,
        params: &SendRichMessageDraftParams,
    ) -> TelegramResult<bool> {
        self.request("sendRichMessageDraft", params).await
    }
}