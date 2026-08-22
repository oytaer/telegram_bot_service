//! 回调查询相关方法
//! 包含 answerCallbackQuery

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

/// answerCallbackQuery 参数
#[derive(Debug, Clone, Serialize)]
pub struct AnswerCallbackQueryParams {
    /// 回调查询唯一 ID（必填）
    pub callback_query_id: String,
    /// 提示文字（选填，0-200字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 是否显示为警告样式（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_alert: Option<bool>,
    /// 点击后跳转的 URL（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 缓存时间（选填，秒，默认0）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
}

impl TelegramClient {
    /// 回答回调查询
    /// 对应官方方法：answerCallbackQuery
    /// 用于响应用户点击内联键盘按钮
    pub async fn answer_callback_query(
        &self,
        params: &AnswerCallbackQueryParams,
    ) -> TelegramResult<bool> {
        self.request("answerCallbackQuery", params).await
    }
}