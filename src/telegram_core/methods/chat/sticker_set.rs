//! 聊天贴纸集相关方法
//! 包含 setChatStickerSet、deleteChatStickerSet

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;

/// setChatStickerSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetChatStickerSetParams {
    /// 目标聊天 ID 或 @username（必填）
    pub chat_id: ChatId,
    /// 贴纸集名称（必填）
    pub sticker_set_name: String,
}

/// deleteChatStickerSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteChatStickerSetParams {
    /// 目标聊天 ID 或 @username（必填）
    pub chat_id: ChatId,
}

impl TelegramClient {
    /// 设置群组贴纸集
    /// 对应官方方法：setChatStickerSet
    pub async fn set_chat_sticker_set(
        &self,
        params: &SetChatStickerSetParams,
    ) -> TelegramResult<bool> {
        self.request("setChatStickerSet", params).await
    }

    /// 删除群组贴纸集
    /// 对应官方方法：deleteChatStickerSet
    pub async fn delete_chat_sticker_set(
        &self,
        params: &DeleteChatStickerSetParams,
    ) -> TelegramResult<bool> {
        self.request("deleteChatStickerSet", params).await
    }
}
