//! 聊天权限相关方法
//! 包含 setChatPermissions

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::chat::ChatPermissions;

/// setChatPermissions 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetChatPermissionsParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 新的默认权限（必填）
    pub permissions: ChatPermissions,
    /// 是否使用独立聊天权限（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_independent_chat_permissions: Option<bool>,
}

impl TelegramClient {
    /// 设置聊天默认成员权限
    /// 对应官方方法：setChatPermissions
    /// 需要 can_restrict_members 权限
    pub async fn set_chat_permissions(
        &self,
        params: &SetChatPermissionsParams,
    ) -> TelegramResult<bool> {
        self.request("setChatPermissions", params).await
    }
}