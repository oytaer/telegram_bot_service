//! Business 连接与 Business Account 相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::methods::messaging::send::media::InputFile;

#[derive(Debug, Clone, Serialize)]
pub struct GetBusinessConnectionParams {
    pub business_connection_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReadBusinessMessageParams {
    pub business_connection_id: String,
    pub chat_id: i64,
    pub message_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteBusinessMessagesParams {
    pub business_connection_id: String,
    pub message_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetBusinessAccountNameParams {
    pub business_connection_id: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetBusinessAccountUsernameParams {
    pub business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetBusinessAccountBioParams {
    pub business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetBusinessAccountProfilePhotoParams {
    pub business_connection_id: String,
    pub photo: InputFile,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoveBusinessAccountProfilePhotoParams {
    pub business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SetBusinessAccountGiftSettingsParams {
    pub business_connection_id: String,
    pub show_gift_button: bool,
    pub accepted_gift_types: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetBusinessAccountStarBalanceParams {
    pub business_connection_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferBusinessAccountStarsParams {
    pub business_connection_id: String,
    pub star_count: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetBusinessAccountGiftsParams {
    pub business_connection_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unsaved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_saved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unlimited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_limited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_unique: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by_price: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl TelegramClient {
    pub async fn get_business_connection(
        &self,
        params: &GetBusinessConnectionParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getBusinessConnection", params).await
    }

    pub async fn read_business_message(
        &self,
        params: &ReadBusinessMessageParams,
    ) -> TelegramResult<bool> {
        self.request("readBusinessMessage", params).await
    }

    pub async fn delete_business_messages(
        &self,
        params: &DeleteBusinessMessagesParams,
    ) -> TelegramResult<bool> {
        self.request("deleteBusinessMessages", params).await
    }

    pub async fn set_business_account_name(
        &self,
        params: &SetBusinessAccountNameParams,
    ) -> TelegramResult<bool> {
        self.request("setBusinessAccountName", params).await
    }

    pub async fn set_business_account_username(
        &self,
        params: &SetBusinessAccountUsernameParams,
    ) -> TelegramResult<bool> {
        self.request("setBusinessAccountUsername", params).await
    }

    pub async fn set_business_account_bio(
        &self,
        params: &SetBusinessAccountBioParams,
    ) -> TelegramResult<bool> {
        self.request("setBusinessAccountBio", params).await
    }

    pub async fn set_business_account_profile_photo(
        &self,
        params: &SetBusinessAccountProfilePhotoParams,
    ) -> TelegramResult<bool> {
        self.request("setBusinessAccountProfilePhoto", params).await
    }

    pub async fn remove_business_account_profile_photo(
        &self,
        params: &RemoveBusinessAccountProfilePhotoParams,
    ) -> TelegramResult<bool> {
        self.request("removeBusinessAccountProfilePhoto", params).await
    }

    pub async fn set_business_account_gift_settings(
        &self,
        params: &SetBusinessAccountGiftSettingsParams,
    ) -> TelegramResult<bool> {
        self.request("setBusinessAccountGiftSettings", params).await
    }

    pub async fn get_business_account_star_balance(
        &self,
        params: &GetBusinessAccountStarBalanceParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getBusinessAccountStarBalance", params).await
    }

    pub async fn transfer_business_account_stars(
        &self,
        params: &TransferBusinessAccountStarsParams,
    ) -> TelegramResult<bool> {
        self.request("transferBusinessAccountStars", params).await
    }

    pub async fn get_business_account_gifts(
        &self,
        params: &GetBusinessAccountGiftsParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getBusinessAccountGifts", params).await
    }
}
