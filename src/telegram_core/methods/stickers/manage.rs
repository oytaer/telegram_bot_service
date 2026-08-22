//! 贴纸集管理相关方法
//! 包含 createNewStickerSet、addStickerToSet、setStickerPositionInSet、deleteStickerFromSet 等

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
    /// 贴纸集所有者用户 ID（必填）
    pub user_id: i64,
    /// 贴纸集名称（必填，需以 _by_<bot username> 结尾）
    pub name: String,
    /// 贴纸集标题（必填，1-64字符）
    pub title: String,
    /// 初始贴纸列表（必填，1-50个）
    pub stickers: Vec<InputSticker>,
    /// 贴纸类型（选填）：regular、mask、custom_emoji
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_type: Option<String>,
    /// 是否需要重绘（选填，仅 custom_emoji）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
}

/// addStickerToSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct AddStickerToSetParams {
    /// 贴纸集所有者用户 ID（必填）
    pub user_id: i64,
    /// 贴纸集名称（必填）
    pub name: String,
    /// 要添加的贴纸（必填）
    pub sticker: InputSticker,
}

/// setStickerPositionInSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerPositionInSetParams {
    /// 贴纸 file_id（必填）
    pub sticker: String,
    /// 新位置（必填，从0开始）
    pub position: i32,
}

/// deleteStickerFromSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteStickerFromSetParams {
    /// 贴纸 file_id（必填）
    pub sticker: String,
}

/// setStickerSetThumbnail 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerSetThumbnailParams {
    /// 贴纸集名称（必填）
    pub name: String,
    /// 所有者用户 ID（必填）
    pub user_id: i64,
    /// 缩略图文件（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<InputFile>,
    /// 格式（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// setStickerSetTitle 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerSetTitleParams {
    /// 贴纸集名称（必填）
    pub name: String,
    /// 新标题（必填）
    pub title: String,
}

/// deleteStickerSet 参数
#[derive(Debug, Clone, Serialize)]
pub struct DeleteStickerSetParams {
    /// 贴纸集名称（必填）
    pub name: String,
}

/// setStickerEmojiList 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerEmojiListParams {
    /// 贴纸 file_id（必填）
    pub sticker: String,
    /// 新 emoji 列表（必填）
    pub emoji_list: Vec<String>,
}

/// setStickerKeywords 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerKeywordsParams {
    /// 贴纸 file_id（必填）
    pub sticker: String,
    /// 关键词列表（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}

/// setStickerMaskPosition 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetStickerMaskPositionParams {
    /// 贴纸 file_id（必填）
    pub sticker: String,
    /// 遮罩位置（选填，传空可移除）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
}

impl TelegramClient {
    /// 创建新贴纸集
    pub async fn create_new_sticker_set(
        &self,
        params: &CreateNewStickerSetParams,
    ) -> TelegramResult<bool> {
        self.request("createNewStickerSet", params).await
    }

    /// 向贴纸集添加贴纸
    pub async fn add_sticker_to_set(
        &self,
        params: &AddStickerToSetParams,
    ) -> TelegramResult<bool> {
        self.request("addStickerToSet", params).await
    }

    /// 设置贴纸在贴纸集中的位置
    pub async fn set_sticker_position_in_set(
        &self,
        params: &SetStickerPositionInSetParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerPositionInSet", params).await
    }

    /// 从贴纸集删除贴纸
    pub async fn delete_sticker_from_set(
        &self,
        params: &DeleteStickerFromSetParams,
    ) -> TelegramResult<bool> {
        self.request("deleteStickerFromSet", params).await
    }

    /// 设置贴纸集缩略图
    pub async fn set_sticker_set_thumbnail(
        &self,
        params: &SetStickerSetThumbnailParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerSetThumbnail", params).await
    }

    /// 设置贴纸集标题
    pub async fn set_sticker_set_title(
        &self,
        params: &SetStickerSetTitleParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerSetTitle", params).await
    }

    /// 删除贴纸集
    pub async fn delete_sticker_set(
        &self,
        params: &DeleteStickerSetParams,
    ) -> TelegramResult<bool> {
        self.request("deleteStickerSet", params).await
    }

    /// 设置贴纸 emoji 列表
    pub async fn set_sticker_emoji_list(
        &self,
        params: &SetStickerEmojiListParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerEmojiList", params).await
    }

    /// 设置贴纸关键词
    pub async fn set_sticker_keywords(
        &self,
        params: &SetStickerKeywordsParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerKeywords", params).await
    }

    /// 设置贴纸遮罩位置
    pub async fn set_sticker_mask_position(
        &self,
        params: &SetStickerMaskPositionParams,
    ) -> TelegramResult<bool> {
        self.request("setStickerMaskPosition", params).await
    }
}