//! 成员控制相关方法（需要管理员权限）
//! 包含 banChatMember、unbanChatMember、restrictChatMember、promoteChatMember 等

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::chat::ChatPermissions;

/// banChatMember 参数
#[derive(Debug, Clone, Serialize)]
pub struct BanChatMemberParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
    /// 解封时间（选填，Unix 时间戳，0 表示永久）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
    /// 是否删除该用户所有消息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoke_messages: Option<bool>,
}

/// unbanChatMember 参数
#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMemberParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
    /// 是否仅移除封禁状态而不重新邀请（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_if_banned: Option<bool>,
}

/// restrictChatMember 参数
#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMemberParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
    /// 新的权限设置（必填）
    pub permissions: ChatPermissions,
    /// 是否使用独立聊天权限（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
    /// 限制结束时间（选填，Unix 时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<i64>,
}

/// promoteChatMember 参数
#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMemberParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
    /// 是否保持匿名（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_anonymous: Option<bool>,
    /// 是否可管理聊天（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_chat: Option<bool>,
    /// 是否可删除消息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    /// 是否可管理视频聊天（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_video_chats: Option<bool>,
    /// 是否可限制成员（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    /// 是否可提升成员（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
    /// 是否可修改聊天信息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    /// 是否可邀请用户（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    /// 是否可发布消息（频道，选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    /// 是否可编辑消息（频道，选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    /// 是否可置顶消息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    /// 是否可管理话题（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

/// setChatAdministratorCustomTitle 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetChatAdministratorCustomTitleParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 目标用户 ID（必填）
    pub user_id: i64,
    /// 自定义头衔（必填，0-16字符）
    pub custom_title: String,
}

impl TelegramClient {
    /// 封禁聊天成员
    /// 对应官方方法：banChatMember
    /// 需要 can_restrict_members 权限
    pub async fn ban_chat_member(&self, params: &BanChatMemberParams) -> TelegramResult<bool> {
        self.request("banChatMember", params).await
    }

    /// 解封聊天成员
    /// 对应官方方法：unbanChatMember
    pub async fn unban_chat_member(&self, params: &UnbanChatMemberParams) -> TelegramResult<bool> {
        self.request("unbanChatMember", params).await
    }

    /// 限制聊天成员权限
    /// 对应官方方法：restrictChatMember
    /// 需要 can_restrict_members 权限
    pub async fn restrict_chat_member(
        &self,
        params: &RestrictChatMemberParams,
    ) -> TelegramResult<bool> {
        self.request("restrictChatMember", params).await
    }

    /// 提升或降级聊天成员
    /// 对应官方方法：promoteChatMember
    /// 需要 can_promote_members 权限
    pub async fn promote_chat_member(
        &self,
        params: &PromoteChatMemberParams,
    ) -> TelegramResult<bool> {
        self.request("promoteChatMember", params).await
    }

    /// 设置管理员自定义头衔
    /// 对应官方方法：setChatAdministratorCustomTitle
    pub async fn set_chat_administrator_custom_title(
        &self,
        params: &SetChatAdministratorCustomTitleParams,
    ) -> TelegramResult<bool> {
        self.request("setChatAdministratorCustomTitle", params).await
    }
}