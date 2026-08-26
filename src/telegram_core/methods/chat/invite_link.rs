//! 聊天邀请链接完整方法
//! 包含 export/create/edit/revoke 以及 subscription 系列

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// ChatInviteLink（返回类型简化）
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ChatInviteLink {
    pub invite_link: String,
    #[serde(default)]
    pub creates_join_request: bool,
    #[serde(default)]
    pub is_primary: bool,
    #[serde(default)]
    pub is_revoked: bool,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub expire_date: Option<i64>,
    #[serde(default)]
    pub member_limit: Option<i32>,
    #[serde(default)]
    pub pending_join_request_count: Option<i32>,
    #[serde(default)]
    pub subscription_period: Option<i32>,
    #[serde(default)]
    pub subscription_price: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ExportChatInviteLinkParams {
    pub chat_id: ChatId,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateChatInviteLinkParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditChatInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CreateChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,
    pub subscription_period: i32,
    pub subscription_price: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditChatSubscriptionInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RevokeChatInviteLinkParams {
    pub chat_id: ChatId,
    pub invite_link: String,
}

impl TelegramClient {
    pub async fn export_chat_invite_link(
        &self,
        params: &ExportChatInviteLinkParams,
    ) -> TelegramResult<String> {
        self.request("exportChatInviteLink", params).await
    }

    pub async fn create_chat_invite_link(
        &self,
        params: &CreateChatInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("createChatInviteLink", params).await
    }

    pub async fn edit_chat_invite_link(
        &self,
        params: &EditChatInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("editChatInviteLink", params).await
    }

    pub async fn create_chat_subscription_invite_link(
        &self,
        params: &CreateChatSubscriptionInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("createChatSubscriptionInviteLink", params).await
    }

    pub async fn edit_chat_subscription_invite_link(
        &self,
        params: &EditChatSubscriptionInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("editChatSubscriptionInviteLink", params).await
    }

    pub async fn revoke_chat_invite_link(
        &self,
        params: &RevokeChatInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("revokeChatInviteLink", params).await
    }
}
