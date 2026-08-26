//! 聊天加入请求相关方法
//! 包含 approveChatJoinRequest、declineChatJoinRequest、
//! answerChatJoinRequestQuery、sendChatJoinRequestWebApp

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// approveChatJoinRequest 参数
#[derive(Debug, Clone, Serialize)]
pub struct ApproveChatJoinRequestParams {
    pub chat_id: ChatId,
    pub user_id: i64,
}

/// declineChatJoinRequest 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeclineChatJoinRequestParams {
    pub chat_id: ChatId,
    pub user_id: i64,
}

/// answerChatJoinRequestQuery 参数
#[derive(Debug, Clone, Serialize)]
pub struct AnswerChatJoinRequestQueryParams {
    pub query_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// sendChatJoinRequestWebApp 参数
#[derive(Debug, Clone, Serialize)]
pub struct SendChatJoinRequestWebAppParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
}

impl TelegramClient {
    pub async fn approve_chat_join_request(
        &self,
        params: &ApproveChatJoinRequestParams,
    ) -> TelegramResult<bool> {
        self.request("approveChatJoinRequest", params).await
    }

    pub async fn decline_chat_join_request(
        &self,
        params: &DeclineChatJoinRequestParams,
    ) -> TelegramResult<bool> {
        self.request("declineChatJoinRequest", params).await
    }

    /// 回答加入请求查询
    pub async fn answer_chat_join_request_query(
        &self,
        params: &AnswerChatJoinRequestQueryParams,
    ) -> TelegramResult<bool> {
        self.request("answerChatJoinRequestQuery", params).await
    }

    /// 向加入请求用户发送 WebApp
    pub async fn send_chat_join_request_web_app(
        &self,
        params: &SendChatJoinRequestWebAppParams,
    ) -> TelegramResult<bool> {
        self.request("sendChatJoinRequestWebApp", params).await
    }
}
