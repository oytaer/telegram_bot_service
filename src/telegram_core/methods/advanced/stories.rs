//! Stories 相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

#[derive(Debug, Clone, Serialize)]
pub struct PostStoryParams {
    pub business_connection_id: String,
    pub content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::telegram_core::types::common::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_chat_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditStoryParams {
    pub business_connection_id: String,
    pub story_id: i32,
    pub content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<crate::telegram_core::types::common::MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areas: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeleteStoryParams {
    pub business_connection_id: String,
    pub story_id: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct RepostStoryParams {
    pub business_connection_id: String,
    pub from_chat_id: i64,
    pub story_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_to_chat_page: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
}

impl TelegramClient {
    pub async fn post_story(&self, params: &PostStoryParams) -> TelegramResult<serde_json::Value> {
        self.request("postStory", params).await
    }

    pub async fn edit_story(&self, params: &EditStoryParams) -> TelegramResult<serde_json::Value> {
        self.request("editStory", params).await
    }

    pub async fn delete_story(&self, params: &DeleteStoryParams) -> TelegramResult<bool> {
        self.request("deleteStory", params).await
    }

    pub async fn repost_story(
        &self,
        params: &RepostStoryParams,
    ) -> TelegramResult<serde_json::Value> {
        self.request("repostStory", params).await
    }
}
