//! 用户资料相关方法
//! 包含 getUserProfilePhotos、getUserProfileAudios、getUserChatBoosts

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

/// UserProfilePhotos
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: i32,
    pub photos: Vec<Vec<serde_json::Value>>,
}

/// UserProfileAudios
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UserProfileAudios {
    pub total_count: i32,
    pub audios: Vec<serde_json::Value>,
}

/// UserChatBoosts
#[derive(Debug, Clone, serde::Deserialize)]
pub struct UserChatBoosts {
    pub boosts: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotosParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfileAudiosParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserChatBoostsParams {
    pub chat_id: crate::telegram_core::types::common::ChatId,
    pub user_id: i64,
}

impl TelegramClient {
    pub async fn get_user_profile_photos(
        &self,
        params: &GetUserProfilePhotosParams,
    ) -> TelegramResult<UserProfilePhotos> {
        self.request("getUserProfilePhotos", params).await
    }

    pub async fn get_user_profile_audios(
        &self,
        params: &GetUserProfileAudiosParams,
    ) -> TelegramResult<UserProfileAudios> {
        self.request("getUserProfileAudios", params).await
    }

    pub async fn get_user_chat_boosts(
        &self,
        params: &GetUserChatBoostsParams,
    ) -> TelegramResult<UserChatBoosts> {
        self.request("getUserChatBoosts", params).await
    }
}
