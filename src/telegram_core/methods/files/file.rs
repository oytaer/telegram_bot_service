//! 文件相关方法
//! 包含 getFile

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

/// 文件对象（官方 File）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// 文件标识符
    pub file_id: String,
    /// 文件唯一标识符
    pub file_unique_id: String,
    /// 文件大小（选填，字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件路径（选填，可通过该路径下载）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

/// getFile 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetFileParams {
    /// 文件标识符（必填）
    pub file_id: String,
}

impl TelegramClient {
    /// 获取文件信息
    /// 对应官方方法：getFile
    /// 成功后可通过 https://api.telegram.org/file/bot<token>/<file_path> 下载
    pub async fn get_file(&self, params: &GetFileParams) -> TelegramResult<File> {
        self.request("getFile", params).await
    }
}