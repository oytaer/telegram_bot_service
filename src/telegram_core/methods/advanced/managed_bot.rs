//! Managed Bot 相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

#[derive(Debug, Clone, Serialize)]
pub struct GetManagedBotTokenParams {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReplaceManagedBotTokenParams {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetManagedBotAccessSettingsParams {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetManagedBotAccessSettingsParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_bots: Option<bool>,
}

impl TelegramClient {
    pub async fn get_managed_bot_token(
        &self,
        params: &GetManagedBotTokenParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getManagedBotToken", params).await
    }

    pub async fn replace_managed_bot_token(
        &self,
        params: &ReplaceManagedBotTokenParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("replaceManagedBotToken", params).await
    }

    pub async fn get_managed_bot_access_settings(
        &self,
        params: &GetManagedBotAccessSettingsParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getManagedBotAccessSettings", params).await
    }

    pub async fn set_managed_bot_access_settings(
        &self,
        params: &SetManagedBotAccessSettingsParams,
    ) -> TelegramResult<bool> {
        self.request("setManagedBotAccessSettings", params).await
    }
}
