//! sendMessage 完整实现
//! 对应官方文档：https://core.telegram.org/bots/api#sendmessage
//! 所有官方参数完整包含，严格区分必填与选填

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::common::{ChatId, MessageEntity};
use crate::telegram_core::types::keyboard::ReplyMarkup;

/// sendMessage 完整参数
#[derive(Debug, Clone, Serialize)]
pub struct SendMessageParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// 目标聊天 ID 或 @username（必填）
    pub chat_id: ChatId,

    /// 消息线程/话题 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    /// 直接消息话题 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,

    /// 临时消息接收者用户 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,

    /// 消息文本（必填）
    pub text: String,

    /// 解析模式（选填）：HTML / Markdown / MarkdownV2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,

    /// 文本实体列表（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// 链接预览选项（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// 是否静默发送（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,

    /// 是否保护内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,

    /// 是否允许付费广播（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,

    /// 消息效果 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,

    /// 建议帖子参数（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_post_parameters: Option<SuggestedPostParameters>,

    /// 回复参数（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,

    /// 回复键盘或内联键盘（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

/// 链接预览选项
#[derive(Debug, Clone, Serialize)]
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

/// 回复参数
#[derive(Debug, Clone, Serialize)]
pub struct ReplyParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_message_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_position: Option<i32>,
}

/// 建议帖子参数
#[derive(Debug, Clone, Serialize)]
pub struct SuggestedPostParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<i64>,
}

impl TelegramClient {
    /// 发送文本消息
    /// 对应官方方法：sendMessage
    pub async fn send_message(&self, params: &SendMessageParams) -> TelegramResult<Message> {
        self.request("sendMessage", params).await
    }
}