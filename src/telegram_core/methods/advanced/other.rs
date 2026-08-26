//! 其他高级方法
//! 包含 getUserPersonalChatMessages、suggested post、prepared message、checklist、star subscription

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::keyboard::InlineKeyboardMarkup;

#[derive(Debug, Clone, Serialize)]
pub struct GetUserPersonalChatMessagesParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ApproveSuggestedPostParams {
    pub chat_id: i64,
    pub message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeclineSuggestedPostParams {
    pub chat_id: i64,
    pub message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SavePreparedInlineMessageParams {
    pub user_id: i64,
    pub result: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SavePreparedKeyboardButtonParams {
    pub user_id: i64,
    pub button: serde_json::Value,
}

#[derive(Debug, Clone, Serialize)]
pub struct SendChecklistParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub checklist: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<crate::telegram_core::methods::messaging::send::text::ReplyParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<crate::telegram_core::types::keyboard::ReplyMarkup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditMessageChecklistParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    pub chat_id: ChatId,
    pub message_id: i32,
    pub checklist: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditUserStarSubscriptionParams {
    pub user_id: i64,
    pub telegram_payment_charge_id: String,
    pub is_canceled: bool,
}

impl TelegramClient {
    pub async fn get_user_personal_chat_messages(
        &self,
        params: &GetUserPersonalChatMessagesParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("getUserPersonalChatMessages", params).await
    }

    pub async fn approve_suggested_post(
        &self,
        params: &ApproveSuggestedPostParams,
    ) -> TelegramResult<bool> {
        self.request("approveSuggestedPost", params).await
    }

    pub async fn decline_suggested_post(
        &self,
        params: &DeclineSuggestedPostParams,
    ) -> TelegramResult<bool> {
        self.request("declineSuggestedPost", params).await
    }

    pub async fn save_prepared_inline_message(
        &self,
        params: &SavePreparedInlineMessageParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("savePreparedInlineMessage", params).await
    }

    pub async fn save_prepared_keyboard_button(
        &self,
        params: &SavePreparedKeyboardButtonParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("savePreparedKeyboardButton", params).await
    }

    pub async fn send_checklist(
        &self,
        params: &SendChecklistParams,
    ) -> TelegramResult<Message> {
        self.request("sendChecklist", params).await
    }

    pub async fn edit_message_checklist(
        &self,
        params: &EditMessageChecklistParams,
    ) -> TelegramResult<Message> {
        self.request("editMessageChecklist", params).await
    }

    pub async fn edit_user_star_subscription(
        &self,
        params: &EditUserStarSubscriptionParams,
    ) -> TelegramResult<bool> {
        self.request("editUserStarSubscription", params).await
    }
}
