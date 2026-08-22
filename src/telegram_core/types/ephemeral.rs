//! 临时消息（Ephemeral Message）相关类型

use serde::{Deserialize, Serialize};
use super::user::User;

/// 临时消息标识
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralMessageId {
    /// 临时消息 ID
    pub ephemeral_message_id: String,
}

/// 临时消息接收者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EphemeralReceiver {
    /// 接收用户
    pub receiver_user: User,
}