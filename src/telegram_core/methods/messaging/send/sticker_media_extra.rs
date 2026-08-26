//! 补充媒体与贴纸发送方法
//! 包含 sendSticker、sendMediaGroup、sendLivePhoto、sendPaidMedia、sendMessageDraft
//! 对应官方文档：https://core.telegram.org/bots/api

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::common::{ChatId, MessageEntity};
use crate::telegram_core::types::keyboard::ReplyMarkup;
use super::media::InputFile;
use super::text::{ReplyParameters, SuggestedPostParameters};

// ==================== InputMedia 类型（用于 sendMediaGroup / editMessageMedia） ====================

/// 输入媒体联合类型
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum InputMedia {
    #[serde(rename = "photo")]
    Photo(InputMediaPhoto),
    #[serde(rename = "video")]
    Video(InputMediaVideo),
    #[serde(rename = "animation")]
    Animation(InputMediaAnimation),
    #[serde(rename = "audio")]
    Audio(InputMediaAudio),
    #[serde(rename = "document")]
    Document(InputMediaDocument),
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaPhoto {
    pub media: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaVideo {
    pub media: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaAnimation {
    pub media: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaAudio {
    pub media: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputMediaDocument {
    pub media: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_content_type_detection: Option<bool>,
}

// ==================== sendSticker ====================

#[derive(Debug, Clone, Serialize)]
pub struct SendStickerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,
    /// 贴纸（file_id / URL / attach://）
    pub sticker: InputFile,
    /// 仅对新上传贴纸有效的 emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
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

// ==================== sendMediaGroup ====================

#[derive(Debug, Clone, Serialize)]
pub struct SendMediaGroupParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,
    /// 媒体数组（2-10 个）
    pub media: Vec<InputMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
}

// ==================== sendLivePhoto ====================

#[derive(Debug, Clone, Serialize)]
pub struct SendLivePhotoParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,
    /// Live Photo 文件
    pub live_photo: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
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

// ==================== InputPaidMedia ====================

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum InputPaidMedia {
    #[serde(rename = "photo")]
    Photo(InputPaidMediaPhoto),
    #[serde(rename = "video")]
    Video(InputPaidMediaVideo),
    #[serde(rename = "live_photo")]
    LivePhoto(InputPaidMediaLivePhoto),
}

#[derive(Debug, Clone, Serialize)]
pub struct InputPaidMediaPhoto {
    pub media: InputFile,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputPaidMediaVideo {
    pub media: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct InputPaidMediaLivePhoto {
    pub media: InputFile,
}

// ==================== sendPaidMedia ====================

#[derive(Debug, Clone, Serialize)]
pub struct SendPaidMediaParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,
    /// 需要支付的 Stars 数量
    pub star_count: i32,
    /// 付费媒体列表
    pub media: Vec<InputPaidMedia>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<SuggestedPostParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

// ==================== sendMessageDraft ====================

#[derive(Debug, Clone, Serialize)]
pub struct SendMessageDraftParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 草稿消息 ID（用于后续更新同一草稿）
    pub draft_id: i64,
    /// 草稿文本
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    /// 是否可被用户停止生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_stop: Option<bool>,
    /// 停止后是否保留已生成内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_on_stop: Option<bool>,
}

impl TelegramClient {
    /// 发送贴纸
    /// 对应官方方法：sendSticker
    pub async fn send_sticker(&self, params: &SendStickerParams) -> TelegramResult<Message> {
        self.request("sendSticker", params).await
    }

    /// 发送媒体组（相册）
    /// 对应官方方法：sendMediaGroup
    pub async fn send_media_group(
        &self,
        params: &SendMediaGroupParams,
    ) -> TelegramResult<Vec<Message>> {
        self.request("sendMediaGroup", params).await
    }

    /// 发送 Live Photo
    /// 对应官方方法：sendLivePhoto
    pub async fn send_live_photo(
        &self,
        params: &SendLivePhotoParams,
    ) -> TelegramResult<Message> {
        self.request("sendLivePhoto", params).await
    }

    /// 发送付费媒体
    /// 对应官方方法：sendPaidMedia
    pub async fn send_paid_media(
        &self,
        params: &SendPaidMediaParams,
    ) -> TelegramResult<Message> {
        self.request("sendPaidMedia", params).await
    }

    /// 发送消息草稿（流式生成）
    /// 对应官方方法：sendMessageDraft
    pub async fn send_message_draft(
        &self,
        params: &SendMessageDraftParams,
    ) -> TelegramResult<bool> {
        self.request("sendMessageDraft", params).await
    }
}
