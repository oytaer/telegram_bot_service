//! 加入请求相关方法
//! 包含 approveChatJoinRequest、declineChatJoinRequest

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// approveChatJoinRequest 参数
#[derive(Debug, Clone, Serialize)]
pub struct ApproveChatJoinRequestParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
}

/// declineChatJoinRequest 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeclineChatJoinRequestParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
}

impl TelegramClient {
    /// 批准加入请求
    /// 对应官方方法：approveChatJoinRequest
    /// 需要 can_invite_users 权限
    pub async fn approve_chat_join_request(
        &self,
        params: &ApproveChatJoinRequestParams,
    ) -> TelegramResult<bool> {
        self.request("approveChatJoinRequest", params).await
    }

    /// 拒绝加入请求
    /// 对应官方方法：declineChatJoinRequest
    /// 需要 can_invite_users 权限
    pub async fn decline_chat_join_request(
        &self,
        params: &DeclineChatJoinRequestParams,
    ) -> TelegramResult<bool> {
        self.request("declineChatJoinRequest", params).await
    }
}