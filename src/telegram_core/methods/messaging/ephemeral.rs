//! Ephemeral（临时）消息编辑与删除方法
//! 对应官方：editEphemeralMessageText/Media/Caption/ReplyMarkup、deleteEphemeralMessage

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::{ChatId, MessageEntity};
use crate::telegram_core::types::keyboard::InlineKeyboardMarkup;
use super::send::sticker_media_extra::InputMedia;
use super::send::text::LinkPreviewOptions;

/// editEphemeralMessageText 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageTextParams {
    pub chat_id: ChatId,
    pub receiver_user_id: i64,
    pub ephemeral_message_id: i32,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    /// 是否使用 rich_message（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich_message: Option<serde_json::Value>,
}

/// editEphemeralMessageMedia 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageMediaParams {
    pub chat_id: ChatId,
    pub receiver_user_id: i64,
    pub ephemeral_message_id: i32,
    pub media: InputMedia,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// editEphemeralMessageCaption 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageCaptionParams {
    pub chat_id: ChatId,
    pub receiver_user_id: i64,
    pub ephemeral_message_id: i32,
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

/// editEphemeralMessageReplyMarkup 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditEphemeralMessageReplyMarkupParams {
    pub chat_id: ChatId,
    pub receiver_user_id: i64,
    pub ephemeral_message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// deleteEphemeralMessage 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteEphemeralMessageParams {
    pub chat_id: ChatId,
    pub receiver_user_id: i64,
    pub ephemeral_message_id: i32,
}

impl TelegramClient {
    pub async fn edit_ephemeral_message_text(
        &self,
        params: &EditEphemeralMessageTextParams,
    ) -> TelegramResult<bool> {
        self.request("editEphemeralMessageText", params).await
    }

    pub async fn edit_ephemeral_message_media(
        &self,
        params: &EditEphemeralMessageMediaParams,
    ) -> TelegramResult<bool> {
        self.request("editEphemeralMessageMedia", params).await
    }

    pub async fn edit_ephemeral_message_caption(
        &self,
        params: &EditEphemeralMessageCaptionParams,
    ) -> TelegramResult<bool> {
        self.request("editEphemeralMessageCaption", params).await
    }

    pub async fn edit_ephemeral_message_reply_markup(
        &self,
        params: &EditEphemeralMessageReplyMarkupParams,
    ) -> TelegramResult<bool> {
        self.request("editEphemeralMessageReplyMarkup", params).await
    }

    pub async fn delete_ephemeral_message(
        &self,
        params: &DeleteEphemeralMessageParams,
    ) -> TelegramResult<bool> {
        self.request("deleteEphemeralMessage", params).await
    }
}
