//! 聊天查询相关方法
//! 包含 getChat、getChatAdministrators、getChatMember、getChatMemberCount

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::chat::{ChatFullInfo, ChatMember};

/// getChat 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetChatParams {
    /// 目标聊天 ID 或用户名（必填）
    pub chat_id: ChatId,
}

/// getChatAdministrators 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetChatAdministratorsParams {
    /// 目标聊天 ID 或用户名（必填）
    pub chat_id: ChatId,
}

/// getChatMember 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetChatMemberParams {
    /// 目标聊天 ID 或用户名（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
}

/// getChatMemberCount 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetChatMemberCountParams {
    /// 目标聊天 ID 或用户名（必填）
    pub chat_id: ChatId,
}

impl TelegramClient {
    /// 获取聊天完整信息
    /// 对应官方方法：getChat
    pub async fn get_chat(&self, params: &GetChatParams) -> TelegramResult<ChatFullInfo> {
        self.request("getChat", params).await
    }

    /// 获取聊天管理员列表
    /// 对应官方方法：getChatAdministrators
    pub async fn get_chat_administrators(
        &self,
        params: &GetChatAdministratorsParams,
    ) -> TelegramResult<Vec<ChatMember>> {
        self.request("getChatAdministrators", params).await
    }

    /// 获取聊天成员信息
    /// 对应官方方法：getChatMember
    pub async fn get_chat_member(&self, params: &GetChatMemberParams) -> TelegramResult<ChatMember> {
        self.request("getChatMember", params).await
    }

    /// 获取聊天成员数量
    /// 对应官方方法：getChatMemberCount
    pub async fn get_chat_member_count(
        &self,
        params: &GetChatMemberCountParams,
    ) -> TelegramResult<i32> {
        self.request("getChatMemberCount", params).await
    }
}