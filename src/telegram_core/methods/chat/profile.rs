//! 聊天资料修改相关方法
//! 包含 setChatTitle、setChatDescription、setChatPhoto、deleteChatPhoto

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::methods::messaging::send::media::InputFile;

/// setChatTitle 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetChatTitleParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 新标题（必填，1-128字符）
    pub title: String,
}

/// setChatDescription 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetChatDescriptionParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 新描述（选填，0-255字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// setChatPhoto 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetChatPhotoParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 新照片（必填）
    pub photo: InputFile,
}

/// deleteChatPhoto 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteChatPhotoParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
}

impl TelegramClient {
    /// 设置聊天标题
    /// 对应官方方法：setChatTitle
    /// 需要 can_change_info 权限
    pub async fn set_chat_title(&self, params: &SetChatTitleParams) -> TelegramResult<bool> {
        self.request("setChatTitle", params).await
    }

    /// 设置聊天描述
    /// 对应官方方法：setChatDescription
    /// 需要 can_change_info 权限
    pub async fn set_chat_description(
        &self,
        params: &SetChatDescriptionParams,
    ) -> TelegramResult<bool> {
        self.request("setChatDescription", params).await
    }

    /// 设置聊天照片
    /// 对应官方方法：setChatPhoto
    /// 需要 can_change_info 权限
    pub async fn set_chat_photo(&self, params: &SetChatPhotoParams) -> TelegramResult<bool> {
        self.request("setChatPhoto", params).await
    }

    /// 删除聊天照片
    /// 对应官方方法：deleteChatPhoto
    /// 需要 can_change_info 权限
    pub async fn delete_chat_photo(
        &self,
        params: &DeleteChatPhotoParams,
    ) -> TelegramResult<bool> {
        self.request("deleteChatPhoto", params).await
    }
}