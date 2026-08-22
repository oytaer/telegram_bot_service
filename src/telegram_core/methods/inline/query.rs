//! 内联查询相关方法
//! 包含 answerInlineQuery

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

/// 内联查询结果（简化常用类型，后续可扩展完整所有结果类型）
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    /// 文章结果
    #[serde(rename = "article")]
    Article {
        id: String,
        title: String,
        input_message_content: InputMessageContent,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<crate::telegram_core::types::keyboard::InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        hide_url: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_url: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        thumbnail_height: Option<i32>,
    },
    /// 照片结果
    #[serde(rename = "photo")]
    Photo {
        id: String,
        photo_url: String,
        thumbnail_url: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        photo_height: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        caption: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        parse_mode: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        reply_markup: Option<crate::telegram_core::types::keyboard::InlineKeyboardMarkup>,
        #[serde(skip_serializing_if = "Option::is_none")]
        input_message_content: Option<InputMessageContent>,
    },
}

/// 输入消息内容
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputTextMessageContent),
}

/// 文本消息内容
#[derive(Debug, Clone, Serialize)]
pub struct InputTextMessageContent {
    pub message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<crate::telegram_core::types::common::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<crate::telegram_core::methods::messaging::send::text::LinkPreviewOptions>,
}

/// answerInlineQuery 参数
#[derive(Debug, Clone, Serialize)]
pub struct AnswerInlineQueryParams {
    /// 内联查询唯一 ID（必填）
    pub inline_query_id: String,
    /// 结果列表（必填，最多50个）
    pub results: Vec<InlineQueryResult>,
    /// 缓存时间（选填，秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// 是否为个人结果（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// 下一次偏移量（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
    /// 切换私聊按钮文字（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button: Option<InlineQueryResultsButton>,
}

/// 内联查询结果按钮
#[derive(Debug, Clone, Serialize)]
pub struct InlineQueryResultsButton {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<crate::telegram_core::types::keyboard::WebAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
}

impl TelegramClient {
    /// 回答内联查询
    /// 对应官方方法：answerInlineQuery
    pub async fn answer_inline_query(
        &self,
        params: &AnswerInlineQueryParams,
    ) -> TelegramResult<bool> {
        self.request("answerInlineQuery", params).await
    }
}