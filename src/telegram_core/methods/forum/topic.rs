//! 论坛话题管理相关方法
//! 包含 createForumTopic、editForumTopic、closeForumTopic、reopenForumTopic、deleteForumTopic 等

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// 论坛话题（官方 ForumTopic）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopic {
    /// 话题消息线程 ID
    pub message_thread_id: i32,
    /// 话题名称
    pub name: String,
    /// 图标颜色
    pub icon_color: i32,
    /// 自定义 emoji 图标 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// createForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct CreateForumTopicParams {
    /// 目标超级群 ID（必填）
    pub chat_id: ChatId,
    /// 话题名称（必填，1-128字符）
    pub name: String,
    /// 图标颜色（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_color: Option<i32>,
    /// 自定义 emoji 图标 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// editForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditForumTopicParams {
    /// 目标超级群 ID（必填）
    pub chat_id: ChatId,
    /// 话题消息线程 ID（必填）
    pub message_thread_id: i32,
    /// 新名称（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 新自定义 emoji 图标 ID（选填，传空字符串可移除）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

/// closeForumTopic / reopenForumTopic / deleteForumTopic 参数
#[derive(Debug, Clone, Serialize)]
pub struct ForumTopicActionParams {
    /// 目标超级群 ID（必填）
    pub chat_id: ChatId,
    /// 话题消息线程 ID（必填）
    pub message_thread_id: i32,
}

/// unpinAllForumTopicMessages 参数
#[derive(Debug, Clone, Serialize)]
pub struct UnpinAllForumTopicMessagesParams {
    /// 目标超级群 ID（必填）
    pub chat_id: ChatId,
    /// 话题消息线程 ID（必填）
    pub message_thread_id: i32,
}

impl TelegramClient {
    /// 创建论坛话题
    /// 对应官方方法：createForumTopic
    /// 需要 can_manage_topics 权限
    pub async fn create_forum_topic(
        &self,
        params: &CreateForumTopicParams,
    ) -> TelegramResult<ForumTopic> {
        self.request("createForumTopic", params).await
    }

    /// 编辑论坛话题
    /// 对应官方方法：editForumTopic
    /// 需要 can_manage_topics 权限
    pub async fn edit_forum_topic(&self, params: &EditForumTopicParams) -> TelegramResult<bool> {
        self.request("editForumTopic", params).await
    }

    /// 关闭论坛话题
    /// 对应官方方法：closeForumTopic
    /// 需要 can_manage_topics 权限
    pub async fn close_forum_topic(
        &self,
        params: &ForumTopicActionParams,
    ) -> TelegramResult<bool> {
        self.request("closeForumTopic", params).await
    }

    /// 重新打开论坛话题
    /// 对应官方方法：reopenForumTopic
    /// 需要 can_manage_topics 权限
    pub async fn reopen_forum_topic(
        &self,
        params: &ForumTopicActionParams,
    ) -> TelegramResult<bool> {
        self.request("reopenForumTopic", params).await
    }

    /// 删除论坛话题
    /// 对应官方方法：deleteForumTopic
    /// 需要 can_manage_topics 权限
    pub async fn delete_forum_topic(
        &self,
        params: &ForumTopicActionParams,
    ) -> TelegramResult<bool> {
        self.request("deleteForumTopic", params).await
    }

    /// 取消话题内所有置顶消息
    /// 对应官方方法：unpinAllForumTopicMessages
    pub async fn unpin_all_forum_topic_messages(
        &self,
        params: &UnpinAllForumTopicMessagesParams,
    ) -> TelegramResult<bool> {
        self.request("unpinAllForumTopicMessages", params).await
    }
}