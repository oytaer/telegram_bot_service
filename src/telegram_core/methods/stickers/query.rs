//! 贴纸查询相关方法
//! 包含 getStickerSet、getCustomEmojiStickers

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::message::Sticker;
use crate::telegram_core::types::sticker::StickerSet;

/// getStickerSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetStickerSetParams {
    /// 贴纸集名称（必填）
    pub name: String,
}

/// getCustomEmojiStickers 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetCustomEmojiStickersParams {
    /// 自定义 emoji ID 列表（必填）
    pub custom_emoji_ids: Vec<String>,
}

impl TelegramClient {
    /// 获取贴纸集信息
    /// 对应官方方法：getStickerSet
    pub async fn get_sticker_set(
        &self,
        params: &GetStickerSetParams,
    ) -> TelegramResult<StickerSet> {
        self.request("getStickerSet", params).await
    }

    /// 获取自定义 emoji 贴纸信息
    /// 对应官方方法：getCustomEmojiStickers
    pub async fn get_custom_emoji_stickers(
        &self,
        params: &GetCustomEmojiStickersParams,
    ) -> TelegramResult<Vec<Sticker>> {
        self.request("getCustomEmojiStickers", params).await
    }
}