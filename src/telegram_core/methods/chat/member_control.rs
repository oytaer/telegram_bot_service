//! 聊天成员控制相关方法
//! 包含 ban/unban/restrict/promote_chat_member、setChatAdministratorCustomTitle、
//! banChatSenderChat、unbanChatSenderChat、setChatMemberTag

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// ChatPermissions（精简版，用于 restrict）
#[derive(Debug, Clone, Serialize)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BanChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub permissions: ChatPermissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMemberParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_direct_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_welcome_messages: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetChatAdministratorCustomTitleParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    pub custom_title: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct BanChatSenderChatParams {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatSenderChatParams {
    pub chat_id: ChatId,
    pub sender_chat_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetChatMemberTagParams {
    pub chat_id: ChatId,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

impl TelegramClient {
    pub async fn ban_chat_member(
        &self,
        params: &BanChatMemberParams,
    ) -> TelegramResult<bool> {
        self.request("banChatMember", params).await
    }

    pub async fn unban_chat_member(
        &self,
        params: &UnbanChatMemberParams,
    ) -> TelegramResult<bool> {
        self.request("unbanChatMember", params).await
    }

    pub async fn restrict_chat_member(
        &self,
        params: &RestrictChatMemberParams,
    ) -> TelegramResult<bool> {
        self.request("restrictChatMember", params).await
    }

    pub async fn promote_chat_member(
        &self,
        params: &PromoteChatMemberParams,
    ) -> TelegramResult<bool> {
        self.request("promoteChatMember", params).await
    }

    pub async fn set_chat_administrator_custom_title(
        &self,
        params: &SetChatAdministratorCustomTitleParams,
    ) -> TelegramResult<bool> {
        self.request("setChatAdministratorCustomTitle", params).await
    }

    pub async fn ban_chat_sender_chat(
        &self,
        params: &BanChatSenderChatParams,
    ) -> TelegramResult<bool> {
        self.request("banChatSenderChat", params).await
    }

    pub async fn unban_chat_sender_chat(
        &self,
        params: &UnbanChatSenderChatParams,
    ) -> TelegramResult<bool> {
        self.request("unbanChatSenderChat", params).await
    }

    pub async fn set_chat_member_tag(
        &self,
        params: &SetChatMemberTagParams,
    ) -> TelegramResult<bool> {
        self.request("setChatMemberTag", params).await
    }
}
