//! 邀请链接相关方法
//! 包含 exportChatInviteLink、createChatInviteLink、editChatInviteLink、revokeChatInviteLink

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::chat::ChatInviteLink;
use crate::telegram_core::types::common::ChatId;

/// exportChatInviteLink 参数
#[derive(Debug, Clone, Serialize)]
pub struct ExportChatInviteLinkParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
}

/// createChatInviteLink 参数
#[derive(Debug, Clone, Serialize)]
pub struct CreateChatInviteLinkParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 链接名称（选填，0-32字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 过期时间（选填，Unix 时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// 成员数量限制（选填，1-99999）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    /// 是否需要加入审批（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

/// editChatInviteLink 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditChatInviteLinkParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 要编辑的邀请链接（必填）
    pub invite_link: String,
    /// 新名称（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 新过期时间（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// 新成员限制（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    /// 是否需要加入审批（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creates_join_request: Option<bool>,
}

/// revokeChatInviteLink 参数
#[derive(Debug, Clone, Serialize)]
pub struct RevokeChatInviteLinkParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 要撤销的邀请链接（必填）
    pub invite_link: String,
}

impl TelegramClient {
    /// 导出（生成）主邀请链接
    /// 对应官方方法：exportChatInviteLink
    /// 需要 can_invite_users 权限
    pub async fn export_chat_invite_link(
        &self,
        params: &ExportChatInviteLinkParams,
    ) -> TelegramResult<String> {
        self.request("exportChatInviteLink", params).await
    }

    /// 创建额外邀请链接
    /// 对应官方方法：createChatInviteLink
    /// 需要 can_invite_users 权限
    pub async fn create_chat_invite_link(
        &self,
        params: &CreateChatInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("createChatInviteLink", params).await
    }

    /// 编辑邀请链接
    /// 对应官方方法：editChatInviteLink
    /// 需要 can_invite_users 权限
    pub async fn edit_chat_invite_link(
        &self,
        params: &EditChatInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("editChatInviteLink", params).await
    }

    /// 撤销邀请链接
    /// 对应官方方法：revokeChatInviteLink
    /// 需要 can_invite_users 权限
    pub async fn revoke_chat_invite_link(
        &self,
        params: &RevokeChatInviteLinkParams,
    ) -> TelegramResult<ChatInviteLink> {
        self.request("revokeChatInviteLink", params).await
    }
}