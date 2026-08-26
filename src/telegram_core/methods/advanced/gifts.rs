//! Gifts 相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

#[derive(Debug, Clone, Serialize)]
pub struct SendGiftParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    pub gift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_for_upgrade: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<crate::telegram_core::types::common::MessageEntity>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GiftPremiumSubscriptionParams {
    pub user_id: i64,
    pub month_count: i32,
    pub star_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<crate::telegram_core::types::common::MessageEntity>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetUserGiftsParams {
    pub user_id: i64,
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

#[derive(Debug, Clone, Serialize)]
pub struct GetChatGiftsParams {
    pub chat_id: ChatId,
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

#[derive(Debug, Clone, Serialize)]
pub struct ConvertGiftToStarsParams {
    pub business_connection_id: String,
    pub owned_gift_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct UpgradeGiftParams {
    pub business_connection_id: String,
    pub owned_gift_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keep_original_details: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferGiftParams {
    pub business_connection_id: String,
    pub owned_gift_id: String,
    pub new_owner_chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub star_count: Option<i32>,
}

impl TelegramClient {
    pub async fn get_available_gifts(&self) -> TelegramResult<serde_json::Value> {
        self.request_empty("getAvailableGifts").await
    }

    pub async fn send_gift(&self, params: &SendGiftParams) -> TelegramResult<bool> {
        self.request("sendGift", params).await
    }

    pub async fn gift_premium_subscription(
        &self,
        params: &GiftPremiumSubscriptionParams,
    ) -> TelegramResult<bool> {
        self.request("giftPremiumSubscription", params).await
    }

    pub async fn get_user_gifts(
        &self,
        params: &GetUserGiftsParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getUserGifts", params).await
    }

    pub async fn get_chat_gifts(
        &self,
        params: &GetChatGiftsParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getChatGifts", params).await
    }

    pub async fn convert_gift_to_stars(
        &self,
        params: &ConvertGiftToStarsParams,
    ) -> TelegramResult<bool> {
        self.request("convertGiftToStars", params).await
    }

    pub async fn upgrade_gift(&self, params: &UpgradeGiftParams) -> TelegramResult<bool> {
        self.request("upgradeGift", params).await
    }

    pub async fn transfer_gift(&self, params: &TransferGiftParams) -> TelegramResult<bool> {
        self.request("transferGift", params).await
    }
}
