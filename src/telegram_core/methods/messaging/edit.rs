//! 消息编辑相关方法
//! 包含 editMessageText、editMessageCaption、editMessageMedia、editMessageReplyMarkup

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::{ChatId, MessageEntity};
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::keyboard::InlineKeyboardMarkup;
use super::send::text::LinkPreviewOptions;

/// editMessageText 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageTextParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// 聊天 ID（选填，inline 消息可不传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// 消息 ID（选填，inline 消息可不传）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// 内联消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// 新文本（必填）
    pub text: String,
    /// 解析模式（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    /// 实体（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// 链接预览选项（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    /// 内联键盘（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// editMessageCaption 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageCaptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// editMessageReplyMarkup 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageReplyMarkupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl TelegramClient {
    /// 编辑消息文本
    /// 对应官方方法：editMessageText
    pub async fn edit_message_text(
        &self,
        params: &EditMessageTextParams,
    ) -> TelegramResult<Message> {
        self.request("editMessageText", params).await
    }

    /// 编辑消息标题
    /// 对应官方方法：editMessageCaption
    pub async fn edit_message_caption(
        &self,
        params: &EditMessageCaptionParams,
    ) -> TelegramResult<Message> {
        self.request("editMessageCaption", params).await
    }

    /// 编辑消息回复标记
    /// 对应官方方法：editMessageReplyMarkup
    pub async fn edit_message_reply_markup(
        &self,
        params: &EditMessageReplyMarkupParams,
    ) -> TelegramResult<Message> {
        self.request("editMessageReplyMarkup", params).await
    }
}