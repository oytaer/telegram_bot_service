//! Bot 命令相关方法
//! 包含 setMyCommands、getMyCommands、deleteMyCommands

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

/// 机器人命令
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotCommand {
    /// 命令名称（1-32字符，仅小写字母、数字、下划线）
    pub command: String,
    /// 命令描述（1-256字符）
    pub description: String,
}

/// 命令作用域类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BotCommandScope {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "all_private_chats")]
    AllPrivateChats,
    #[serde(rename = "all_group_chats")]
    AllGroupChats,
    #[serde(rename = "all_chat_administrators")]
    AllChatAdministrators,
    #[serde(rename = "chat")]
    Chat { chat_id: crate::telegram_core::types::common::ChatId },
    #[serde(rename = "chat_administrators")]
    ChatAdministrators { chat_id: crate::telegram_core::types::common::ChatId },
    #[serde(rename = "chat_member")]
    ChatMember {
        chat_id: crate::telegram_core::types::common::ChatId,
        user_id: i64,
    },
}

/// setMyCommands 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMyCommandsParams {
    /// 命令列表（必填，最多100个）
    pub commands: Vec<BotCommand>,
    /// 作用域（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    /// 语言代码（选填，IETF 语言标签）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// getMyCommands / deleteMyCommands 参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyCommandsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

impl TelegramClient {
    /// 设置机器人命令列表
    pub async fn set_my_commands(&self, params: &SetMyCommandsParams) -> TelegramResult<bool> {
        self.request("setMyCommands", params).await
    }

    /// 获取机器人命令列表
    pub async fn get_my_commands(&self, params: &GetMyCommandsParams) -> TelegramResult<Vec<BotCommand>> {
        self.request("getMyCommands", params).await
    }

    /// 删除机器人命令列表
    pub async fn delete_my_commands(&self, params: &GetMyCommandsParams) -> TelegramResult<bool> {
        self.request("deleteMyCommands", params).await
    }
}