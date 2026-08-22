//! 用户相关文件与资料方法
//! 包含 getUserProfilePhotos

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::message::PhotoSize;

/// 用户头像照片（官方 UserProfilePhotos）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfilePhotos {
    /// 总照片数量
    pub total_count: i32,
    /// 照片列表（每个元素是一组不同尺寸的照片）
    pub photos: Vec<Vec<PhotoSize>>,
}

/// getUserProfilePhotos 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotosParams {
    /// 目标用户 ID（必填）
    pub user_id: i64,
    /// 偏移量（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 限制数量（选填，1-100，默认100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

impl TelegramClient {
    /// 获取用户头像照片
    /// 对应官方方法：getUserProfilePhotos
    pub async fn get_user_profile_photos(
        &self,
        params: &GetUserProfilePhotosParams,
    ) -> TelegramResult<UserProfilePhotos> {
        self.request("getUserProfilePhotos", params).await
    }
}