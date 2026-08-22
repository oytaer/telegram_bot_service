//! 聊天动作相关方法
//! 包含 sendChatAction

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// sendChatAction 参数
#[derive(Debug, Clone, Serialize)]
pub struct SendChatActionParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,

    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,

    /// 消息线程/话题 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    /// 动作类型（必填）
    /// 可选值：typing、upload_photo、record_video、upload_video、
    /// record_voice、upload_voice、upload_document、choose_sticker、
    /// find_location、record_video_note、upload_video_note
    pub action: String,
}

impl TelegramClient {
    /// 发送聊天动作（例如“正在输入...”）
    /// 对应官方方法：sendChatAction
    pub async fn send_chat_action(
        &self,
        params: &SendChatActionParams,
    ) -> TelegramResult<bool> {
        self.request("sendChatAction", params).await
    }
}