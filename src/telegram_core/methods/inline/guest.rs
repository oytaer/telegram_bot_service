//! Guest 模式相关方法
//! 包含 answerGuestQuery

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::methods::inline::query::InlineQueryResult;

/// answerGuestQuery 参数
#[derive(Debug, Clone, Serialize)]
pub struct AnswerGuestQueryParams {
    /// Guest 查询唯一 ID（必填）
    pub guest_query_id: String,
    /// 结果列表（必填）
    pub results: Vec<InlineQueryResult>,
    /// 缓存时间（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_time: Option<i32>,
    /// 是否为个人结果（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_personal: Option<bool>,
    /// 下一次偏移（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_offset: Option<String>,
}

impl TelegramClient {
    /// 回答 Guest 查询
    /// 对应官方方法：answerGuestQuery
    pub async fn answer_guest_query(
        &self,
        params: &AnswerGuestQueryParams,
    ) -> TelegramResult<bool> {
        self.request("answerGuestQuery", params).await
    }
}