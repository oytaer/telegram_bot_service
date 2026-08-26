//! Forum 话题完整方法
//! 包含 create/edit/close/reopen/delete_forum_topic 以及通用主题相关方法

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::forum::ForumTopic;

/// createForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct CreateForumTopicParams {
    pub chat_id: ChatId,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// editForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// closeForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct CloseForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

/// reopenForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct ReopenForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

/// deleteForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteForumTopicParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

/// unpinAllForumTopicMessages 参数
#[derive(Debug, Clone, Serialize)]
pub struct UnpinAllForumTopicMessagesParams {
    pub chat_id: ChatId,
    pub message_thread_id: i32,
}

/// editGeneralForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditGeneralForumTopicParams {
    pub chat_id: ChatId,
    pub name: String,
}

/// closeGeneralForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct CloseGeneralForumTopicParams {
    pub chat_id: ChatId,
}

/// reopenGeneralForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct ReopenGeneralForumTopicParams {
    pub chat_id: ChatId,
}

/// hideGeneralForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct HideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

/// unhideGeneralForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct UnhideGeneralForumTopicParams {
    pub chat_id: ChatId,
}

/// unpinAllGeneralForumTopicMessages 参数
#[derive(Debug, Clone, Serialize)]
pub struct UnpinAllGeneralForumTopicMessagesParams {
    pub chat_id: ChatId,
}

impl TelegramClient {
    pub async fn create_forum_topic(
        &self,
        params: &CreateForumTopicParams,
    ) -> TelegramResult<ForumTopic> {
        self.request("createForumTopic", params).await
    }

    pub async fn edit_forum_topic(
        &self,
        params: &EditForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("editForumTopic", params).await
    }

    pub async fn close_forum_topic(
        &self,
        params: &CloseForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("closeForumTopic", params).await
    }

    pub async fn reopen_forum_topic(
        &self,
        params: &ReopenForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("reopenForumTopic", params).await
    }

    pub async fn delete_forum_topic(
        &self,
        params: &DeleteForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("deleteForumTopic", params).await
    }

    pub async fn unpin_all_forum_topic_messages(
        &self,
        params: &UnpinAllForumTopicMessagesParams,
    ) -> TelegramResult<bool> {
        self.request("unpinAllForumTopicMessages", params).await
    }

    /// 编辑通用论坛主题名称
    pub async fn edit_general_forum_topic(
        &self,
        params: &EditGeneralForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("editGeneralForumTopic", params).await
    }

    pub async fn close_general_forum_topic(
        &self,
        params: &CloseGeneralForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("closeGeneralForumTopic", params).await
    }

    pub async fn reopen_general_forum_topic(
        &self,
        params: &ReopenGeneralForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("reopenGeneralForumTopic", params).await
    }

    pub async fn hide_general_forum_topic(
        &self,
        params: &HideGeneralForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("hideGeneralForumTopic", params).await
    }

    pub async fn unhide_general_forum_topic(
        &self,
        params: &UnhideGeneralForumTopicParams,
    ) -> TelegramResult<bool> {
        self.request("unhideGeneralForumTopic", params).await
    }

    pub async fn unpin_all_general_forum_topic_messages(
        &self,
        params: &UnpinAllGeneralForumTopicMessagesParams,
    ) -> TelegramResult<bool> {
        self.request("unpinAllGeneralForumTopicMessages", params).await
    }
}
