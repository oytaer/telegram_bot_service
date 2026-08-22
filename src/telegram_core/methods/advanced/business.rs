//! 商业账号相关方法

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::user::User;

/// 商业连接（官方 BusinessConnection）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessConnection {
    pub id: String,
    pub user: User,
    pub user_chat_id: i64,
    pub date: i64,
    pub can_reply: bool,
    pub is_enabled: bool,
}

/// getBusinessConnection 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetBusinessConnectionParams {
    /// 商业连接 ID（必填）
    pub business_connection_id: String,
}

impl TelegramClient {
    /// 获取商业连接信息
    /// 对应官方方法：getBusinessConnection
    pub async fn get_business_connection(
        &self,
        params: &GetBusinessConnectionParams,
    ) -> TelegramResult<BusinessConnection> {
        self.request("getBusinessConnection", params).await
    }
}