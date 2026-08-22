//! 其他高级方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::message::Message;

/// getUserPersonalChatMessages 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetUserPersonalChatMessagesParams {
    /// 用户 ID（必填）
    pub user_id: i64,
    /// 偏移消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_id: Option<i32>,
    /// 限制数量（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl TelegramClient {
    /// 获取用户个人聊天消息（特定场景使用）
    /// 对应官方方法：getUserPersonalChatMessages
    pub async fn get_user_personal_chat_messages(
        &self,
        params: &GetUserPersonalChatMessagesParams,
    ) -> TelegramResult<Vec<Message>> {
        self.request("getUserPersonalChatMessages", params).await
    }
}