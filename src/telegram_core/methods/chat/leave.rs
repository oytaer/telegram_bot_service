//! 离开聊天相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// leaveChat 参数
#[derive(Debug, Clone, Serialize)]
pub struct LeaveChatParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
}

impl TelegramClient {
    /// 机器人离开群组、超级群或频道
    /// 对应官方方法：leaveChat
    pub async fn leave_chat(&self, params: &LeaveChatParams) -> TelegramResult<bool> {
        self.request("leaveChat", params).await
    }
}