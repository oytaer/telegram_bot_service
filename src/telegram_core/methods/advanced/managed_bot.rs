//! 托管 Bot 相关方法

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

/// Bot 访问设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotAccessSettings {
    /// 是否允许加入群组
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_join_groups: Option<bool>,
    /// 是否可读取所有群消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_read_all_group_messages: Option<bool>,
    /// 是否支持内联查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_inline_queries: Option<bool>,
}

/// getManagedBotAccessSettings 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetManagedBotAccessSettingsParams {
    /// 被托管 Bot 的用户 ID（必填）
    pub bot_user_id: i64,
}

/// setManagedBotAccessSettings 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetManagedBotAccessSettingsParams {
    /// 被托管 Bot 的用户 ID（必填）
    pub bot_user_id: i64,
    /// 访问设置（必填）
    pub settings: BotAccessSettings,
}

impl TelegramClient {
    /// 获取托管 Bot 访问设置
    pub async fn get_managed_bot_access_settings(
        &self,
        params: &GetManagedBotAccessSettingsParams,
    ) -> TelegramResult<BotAccessSettings> {
        self.request("getManagedBotAccessSettings", params).await
    }

    /// 设置托管 Bot 访问设置
    pub async fn set_managed_bot_access_settings(
        &self,
        params: &SetManagedBotAccessSettingsParams,
    ) -> TelegramResult<bool> {
        self.request("setManagedBotAccessSettings", params).await
    }
}