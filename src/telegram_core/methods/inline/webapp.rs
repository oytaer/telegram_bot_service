//! Web App 相关方法
//! 包含 answerWebAppQuery

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::methods::inline::query::InlineQueryResult;

/// 发送的 Web App 消息（官方 SentWebAppMessage）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// 已发送内联消息的标识符（选填，仅当消息附有内联键盘时可用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// answerWebAppQuery 参数
#[derive(Debug, Clone, Serialize)]
pub struct AnswerWebAppQueryParams {
    /// Web App 查询唯一 ID（必填）
    pub web_app_query_id: String,
    /// 要发送的结果（必填）
    pub result: InlineQueryResult,
}

impl TelegramClient {
    /// 回答 Web App 查询
    /// 对应官方方法：answerWebAppQuery
    pub async fn answer_web_app_query(
        &self,
        params: &AnswerWebAppQueryParams,
    ) -> TelegramResult<SentWebAppMessage> {
        self.request("answerWebAppQuery", params).await
    }
}