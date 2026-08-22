//! 贴纸相关独立类型定义

use serde::{Deserialize, Serialize};
use super::message::{PhotoSize, MaskPosition};

/// 贴纸集（官方 StickerSet）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StickerSet {
    /// 贴纸集名称
    pub name: String,
    /// 贴纸集标题
    pub title: String,
    /// 贴纸类型：regular、mask、custom_emoji
    #[serde(rename = "sticker_type")]
    pub sticker_type: String,
    /// 是否为动画贴纸
    pub is_animated: bool,
    /// 是否为视频贴纸
    pub is_video: bool,
    /// 贴纸列表
    pub stickers: Vec<super::message::Sticker>,
    /// 缩略图（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

/// 输入贴纸（用于创建/添加贴纸）
#[derive(Debug, Clone, Serialize)]
pub struct InputSticker {
    /// 贴纸文件
    pub sticker: String,
    /// 格式：static、animated、video
    pub format: String,
    /// emoji 列表
    pub emoji_list: Vec<String>,
    /// 遮罩位置（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    /// 关键词（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
}