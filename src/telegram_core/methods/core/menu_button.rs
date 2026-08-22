//! 聊天菜单按钮相关方法
//! 包含 setChatMenuButton、getChatMenuButton

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::keyboard::WebAppInfo;

/// 菜单按钮类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuButton {
    /// 默认按钮
    #[serde(rename = "default")]
    Default,
    /// 命令列表按钮
    #[serde(rename = "commands")]
    Commands,
    /// Web App 按钮
    #[serde(rename = "web_app")]
    WebApp {
        text: String,
        web_app: WebAppInfo,
    },
}

/// setChatMenuButton 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetChatMenuButtonParams {
    /// 目标私聊 ID（选填，不传则设置默认）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// 菜单按钮（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_button: Option<MenuButton>,
}

/// getChatMenuButton 参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetChatMenuButtonParams {
    /// 目标私聊 ID（选填，不传则获取默认）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
}

impl TelegramClient {
    /// 设置聊天菜单按钮
    /// 对应官方方法：setChatMenuButton
    pub async fn set_chat_menu_button(
        &self,
        params: &SetChatMenuButtonParams,
    ) -> TelegramResult<bool> {
        self.request("setChatMenuButton", params).await
    }

    /// 获取聊天菜单按钮
    /// 对应官方方法：getChatMenuButton
    pub async fn get_chat_menu_button(
        &self,
        params: &GetChatMenuButtonParams,
    ) -> TelegramResult<MenuButton> {
        self.request("getChatMenuButton", params).await
    }
}