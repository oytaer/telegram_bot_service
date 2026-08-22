//! 文件相关类型定义

use serde::{Deserialize, Serialize};

/// 官方 File 对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    /// 文件标识符，可用于下载或重用
    pub file_id: String,
    /// 文件唯一标识符，长期有效
    pub file_unique_id: String,
    /// 文件大小（字节，选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件路径（选填，可通过此路径下载）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}