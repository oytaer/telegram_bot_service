//! 用户/聊天验证相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

#[derive(Debug, Clone, Serialize)]
pub struct VerifyUserParams {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyChatParams {
    pub chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoveUserVerificationParams {
    pub user_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct RemoveChatVerificationParams {
    pub chat_id: ChatId,
}

impl TelegramClient {
    pub async fn verify_user(&self, params: &VerifyUserParams) -> TelegramResult<bool> {
        self.request("verifyUser", params).await
    }

    pub async fn verify_chat(&self, params: &VerifyChatParams) -> TelegramResult<bool> {
        self.request("verifyChat", params).await
    }

    pub async fn remove_user_verification(
        &self,
        params: &RemoveUserVerificationParams,
    ) -> TelegramResult<bool> {
        self.request("removeUserVerification", params).await
    }

    pub async fn remove_chat_verification(
        &self,
        params: &RemoveChatVerificationParams,
    ) -> TelegramResult<bool> {
        self.request("removeChatVerification", params).await
    }
}
