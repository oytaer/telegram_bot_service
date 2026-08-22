//! 更新获取相关方法
//! 包含 getUpdates、setWebhook、deleteWebhook、getWebhookInfo

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::update::Update;

/// getUpdates 参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetUpdatesParams {
    /// 偏移量（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,

    /// 限制返回数量（选填，1-100，默认100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,

    /// 长轮询超时时间（选填，0-50秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i32>,

    /// 允许接收的更新类型列表（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

/// setWebhook 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetWebhookParams {
    /// Webhook URL（必填，空字符串可移除）
    pub url: String,

    /// 证书公钥内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<String>,

    /// IP 地址（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// 最大连接数（选填，1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,

    /// 允许的更新类型（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,

    /// 是否丢弃待处理更新（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,

    /// 密钥令牌（选填，1-256字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_token: Option<String>,
}

/// deleteWebhook 参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct DeleteWebhookParams {
    /// 是否丢弃待处理更新（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drop_pending_updates: Option<bool>,
}

/// Webhook 信息（官方 WebhookInfo）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL（空表示未设置）
    pub url: String,
    /// 是否有自定义证书
    pub has_custom_certificate: bool,
    /// 待处理更新数量
    pub pending_update_count: i32,
    /// 当前使用的 IP 地址（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// 最后错误发生时间（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_date: Option<i64>,
    /// 最后错误信息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error_message: Option<String>,
    /// 最后同步错误时间（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_synchronization_error_date: Option<i64>,
    /// 最大连接数（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    /// 允许的更新类型（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<Vec<String>>,
}

impl TelegramClient {
    /// 使用长轮询获取更新
    /// 对应官方方法：getUpdates
    pub async fn get_updates(&self, params: &GetUpdatesParams) -> TelegramResult<Vec<Update>> {
        self.request("getUpdates", params).await
    }

    /// 设置 Webhook
    /// 对应官方方法：setWebhook
    pub async fn set_webhook(&self, params: &SetWebhookParams) -> TelegramResult<bool> {
        self.request("setWebhook", params).await
    }

    /// 删除 Webhook
    /// 对应官方方法：deleteWebhook
    pub async fn delete_webhook(&self, params: &DeleteWebhookParams) -> TelegramResult<bool> {
        self.request("deleteWebhook", params).await
    }

    /// 获取当前 Webhook 状态
    /// 对应官方方法：getWebhookInfo
    pub async fn get_webhook_info(&self) -> TelegramResult<WebhookInfo> {
        self.request_empty("getWebhookInfo").await
    }
}