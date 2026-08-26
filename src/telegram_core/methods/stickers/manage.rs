//! 贴纸集管理相关方法
//! 包含 createNewStickerSet、addStickerToSet、setStickerPositionInSet、deleteStickerFromSet、
//! replaceStickerInSet、setCustomEmojiStickerSetThumbnail 等

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::methods::messaging::send::media::InputFile;
use crate::telegram_core::types::message::MaskPosition;

/// 输入贴纸
#[derive(Debug, Clone, Serialize)]
pub struct InputSticker {
    /// 贴纸文件（file_id、URL 或 attach://）
    pub sticker: InputFile,
    /// 贴纸格式：static、animated、video
    pub format: String,
    /// emoji 列表（1-20个）
    pub emoji_list: Vec<String>,
    /// 遮罩位置（选填，仅遮罩贴纸）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// 关键词列表（选填，0-20个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

/// createNewStickerSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct CreateNewStickerSetParams {
    pub user_id: i64,
    pub name: String,
    pub title: String,
    pub stickers: Vec<InputSticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

/// addStickerToSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct AddStickerToSetParams {
    pub user_id: i64,
    pub name: String,
    pub sticker: InputSticker,
}

/// replaceStickerInSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct ReplaceStickerInSetParams {
    pub user_id: i64,
    pub name: String,
    /// 要被替换的旧贴纸 file_id
    pub old_sticker: String,
    pub sticker: InputSticker,
}

/// setStickerPositionInSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerPositionInSetParams {
    pub sticker: String,
    pub position: i32,
}

/// deleteStickerFromSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteStickerFromSetParams {
    pub sticker: String,
}

/// setStickerSetThumbnail 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerSetThumbnailParams {
    pub name: String,
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// setCustomEmojiStickerSetThumbnail 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetCustomEmojiStickerSetThumbnailParams {
    pub name: String,
    /// 自定义 emoji 的 ID，传空字符串可删除缩略图
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
}

/// setStickerSetTitle 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerSetTitleParams {
    pub name: String,
    pub title: String,
}

/// deleteStickerSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteStickerSetParams {
    pub name: String,
}

/// setStickerEmojiList 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerEmojiListParams {
    pub sticker: String,
    pub emoji_list: Vec<String>,
}

/// setStickerKeywords 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerKeywordsParams {
    pub sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

/// setStickerMaskPosition 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerMaskPositionParams {
    pub sticker: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

impl TelegramClient {
    pub async fn create_new_sticker_set(
        &self,
        params: &CreateNewStickerSetParams,
    ) -> TelegramResult<bool> {
        self.request("createNewStickerSet", params).await
    }

    pub async fn add_sticker_to_set(
        &self,
        params: &AddStickerToSetParams,
    ) -> TelegramResult<bool> {
        self.request("addStickerToSet", params).await
    }

    /// 替换贴纸集中的贴纸
    /// 对应官方方法：replaceStickerInSet
    pub async fn replace_sticker_in_set(
        &self,
        params: &ReplaceStickerInSetParams,
    ) -> TelegramResult<bool> {
        self.request("replaceStickerInSet", params).await
    }

    pub async fn set_sticker_position_in_set(
        &self,
        params: &SetStickerPositionInSetParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerPositionInSet", params).await
    }

    pub async fn delete_sticker_from_set(
        &self,
        params: &DeleteStickerFromSetParams,
    ) -> TelegramResult<bool> {
        self.request("deleteStickerFromSet", params).await
    }

    pub async fn set_sticker_set_thumbnail(
        &self,
        params: &SetStickerSetThumbnailParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerSetThumbnail", params).await
    }

    /// 设置自定义 emoji 贴纸集缩略图
    /// 对应官方方法：setCustomEmojiStickerSetThumbnail
    pub async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        params: &SetCustomEmojiStickerSetThumbnailParams,
    ) -> TelegramResult<bool> {
        self.request("setCustomEmojiStickerSetThumbnail", params).await
    }

    pub async fn set_sticker_set_title(
        &self,
        params: &SetStickerSetTitleParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerSetTitle", params).await
    }

    pub async fn delete_sticker_set(
        &self,
        params: &DeleteStickerSetParams,
    ) -> TelegramResult<bool> {
        self.request("deleteStickerSet", params).await
    }

    pub async fn set_sticker_emoji_list(
        &self,
        params: &SetStickerEmojiListParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerEmojiList", params).await
    }

    pub async fn set_sticker_keywords(
        &self,
        params: &SetStickerKeywordsParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerKeywords", params).await
    }

    pub async fn set_sticker_mask_position(
        &self,
        params: &SetStickerMaskPositionParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerMaskPosition", params).await
    }
}
