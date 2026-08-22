//! 贴纸上传相关方法
//! 包含 uploadStickerFile

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::methods::messaging::send::media::InputFile;
use crate::telegram_core::methods::files::file::File;

/// uploadStickerFile 参数
#[derive(Debug, Clone, Serialize)]
pub struct UploadStickerFileParams {
    /// 贴纸所有者用户 ID（必填）
    pub user_id: i64,
    /// 贴纸文件（必填）
    pub sticker: InputFile,
    /// 贴纸格式（必填）：static、animated、video
    pub sticker_format: String,
}

impl TelegramClient {
    /// 上传贴纸文件
    /// 对应官方方法：uploadStickerFile
    /// 返回 File 对象，可用于后续创建贴纸集
    pub async fn upload_sticker_file(
        &self,
        params: &UploadStickerFileParams,
    ) -> TelegramResult<File> {
        self.request("uploadStickerFile", params).await
    }
}