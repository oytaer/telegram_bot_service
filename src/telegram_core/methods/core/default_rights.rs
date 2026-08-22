//! 默认管理员权限相关方法
//! 包含 setMyDefaultAdministratorRights、getMyDefaultAdministratorRights

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::keyboard::ChatAdministratorRights;

/// setMyDefaultAdministratorRights 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMyDefaultAdministratorRightsParams {
    /// 默认权限（选填，不传则清除）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<ChatAdministratorRights>,
    /// 是否针对频道（选填，默认false针对群组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

/// getMyDefaultAdministratorRights 参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetMyDefaultAdministratorRightsParams {
    /// 是否针对频道（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_channels: Option<bool>,
}

impl TelegramClient {
    /// 设置机器人默认管理员权限建议
    /// 对应官方方法：setMyDefaultAdministratorRights
    pub async fn set_my_default_administrator_rights(
        &self,
        params: &SetMyDefaultAdministratorRightsParams,
    ) -> TelegramResult<bool> {
        self.request("setMyDefaultAdministratorRights", params).await
    }

    /// 获取机器人默认管理员权限建议
    /// 对应官方方法：getMyDefaultAdministratorRights
    pub async fn get_my_default_administrator_rights(
        &self,
        params: &GetMyDefaultAdministratorRightsParams,
    ) -> TelegramResult<ChatAdministratorRights> {
        self.request("getMyDefaultAdministratorRights", params).await
    }
}