//! 互动消息发送方法
//! 包含 sendPoll、sendDice

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::{ChatId, MessageEntity};
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::keyboard::ReplyMarkup;
use super::text::{ReplyParameters, SuggestedPostParameters};

/// 投票选项输入
#[derive(Debug, Clone, Serialize)]
pub struct InputPollOption {
    /// 选项文字（必填）
    pub text: String,
    /// 解析模式（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    /// 实体（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
}

/// sendPoll 参数
#[derive(Debug, Clone, Serialize)]
pub struct SendPollParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,
    /// 问题文字（必填）
    pub question: String,
    /// 问题解析模式（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_parse_mode: Option<String>,
    /// 问题实体（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    /// 选项列表（必填，2-12个）
    pub options: Vec<InputPollOption>,
    /// 是否匿名（选填，默认true）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// 投票类型（选填，quiz 或 regular）
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    /// 是否允许多选（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allows_multiple_answers: Option<bool>,
    /// 正确答案选项 ID（quiz 模式必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i32>,
    /// 解释文字（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    /// 解释解析模式（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_parse_mode: Option<String>,
    /// 解释实体（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    /// 开放时长（选填，秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i32>,
    /// 关闭时间（选填，Unix 时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
    /// 是否立即关闭（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_closed: Option<bool>,
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

/// sendDice 参数
#[derive(Debug, Clone, Serialize)]
pub struct SendDiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_messages_topic_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver_user_id: Option<i64>,
    /// 骰子 emoji（选填，默认🎲）
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

impl TelegramClient {
    /// 发送投票
    pub async fn send_poll(&self, params: &SendPollParams) -> TelegramResult<Message> {
        self.request("sendPoll", params).await
    }

    /// 发送骰子
    pub async fn send_dice(&self, params: &SendDiceParams) -> TelegramResult<Message> {
        self.request("sendDice", params).await
    }
}