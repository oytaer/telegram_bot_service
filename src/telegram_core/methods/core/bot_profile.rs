//! Bot 自身资料相关方法
//! 包含 setMyName/getMyName、setMyDescription/getMyDescription、
//! setMyShortDescription/getMyShortDescription、setMyProfilePhoto/removeMyProfilePhoto、setUserEmojiStatus

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::methods::messaging::send::media::InputFile;

/// BotName
#[derive(Debug, Clone, serde::Deserialize)]
pub struct BotName {
    pub name: String,
}

/// BotDescription
#[derive(Debug, Clone, serde::Deserialize)]
pub struct BotDescription {
    pub description: String,
}

/// BotShortDescription
#[derive(Debug, Clone, serde::Deserialize)]
pub struct BotShortDescription {
    pub short_description: String,
}

/// setMyName 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// getMyName 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetMyNameParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// setMyDescription 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// getMyDescription 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetMyDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// setMyShortDescription 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// getMyShortDescription 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetMyShortDescriptionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// setMyProfilePhoto 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetMyProfilePhotoParams {
    pub photo: InputFile,
}

/// setUserEmojiStatus 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetUserEmojiStatusParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
}

impl TelegramClient {
    pub async fn set_my_name(&self, params: &SetMyNameParams) -> TelegramResult<bool> {
        self.request("setMyName", params).await
    }

    pub async fn get_my_name(&self, params: &GetMyNameParams) -> TelegramResult<BotName> {
        self.request("getMyName", params).await
    }

    pub async fn set_my_description(
        &self,
        params: &SetMyDescriptionParams,
    ) -> TelegramResult<bool> {
        self.request("setMyDescription", params).await
    }

    pub async fn get_my_description(
        &self,
        params: &GetMyDescriptionParams,
    ) -> TelegramResult<BotDescription> {
        self.request("getMyDescription", params).await
    }

    pub async fn set_my_short_description(
        &self,
        params: &SetMyShortDescriptionParams,
    ) -> TelegramResult<bool> {
        self.request("setMyShortDescription", params).await
    }

    pub async fn get_my_short_description(
        &self,
        params: &GetMyShortDescriptionParams,
    ) -> TelegramResult<BotShortDescription> {
        self.request("getMyShortDescription", params).await
    }

    pub async fn set_my_profile_photo(
        &self,
        params: &SetMyProfilePhotoParams,
    ) -> TelegramResult<bool> {
        self.request("setMyProfilePhoto", params).await
    }

    pub async fn remove_my_profile_photo(&self) -> TelegramResult<bool> {
        self.request_empty("removeMyProfilePhoto").await
    }

    pub async fn set_user_emoji_status(
        &self,
        params: &SetUserEmojiStatusParams,
    ) -> TelegramResult<bool> {
        self.request("setUserEmojiStatus", params).await
    }
}
