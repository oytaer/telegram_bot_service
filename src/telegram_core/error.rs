//! Telegram Bot API 统一错误类型定义
//! 用于封装所有可能出现的错误情况

use thiserror::Error; // 使用 thiserror 宏简化错误定义
use serde::Deserialize; // 用于反序列化 Telegram 返回的错误 JSON

/// Telegram API 返回的错误结构体（官方错误格式）
#[derive(Debug, Deserialize, Clone)]
pub struct TelegramApiError {
    /// 是否成功（失败时为 false）
    pub ok: bool,
    /// 错误描述信息
    pub description: Option<String>,
    /// 错误代码（官方定义的 error_code）
    pub error_code: Option<i32>,
    /// 可选的额外参数（例如 retry_after）
    pub parameters: Option<ResponseParameters>,
}

/// 错误响应中可能携带的额外参数
#[derive(Debug, Deserialize, Clone)]
pub struct ResponseParameters {
    /// 如果触发限流，建议等待的秒数
    pub retry_after: Option<i32>,
    /// 迁移到的超级群 ID（当群组升级为超级群时）
    pub migrate_to_chat_id: Option<i64>,
}

/// 项目统一的错误枚举
#[derive(Error, Debug)]
pub enum TelegramError {
    /// HTTP 请求相关错误（网络、超时等）
    #[error("HTTP 请求失败: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON 序列化/反序列化错误
    #[error("JSON 处理失败: {0}")]
    Json(#[from] serde_json::Error),

    /// Telegram API 返回的业务错误
    #[error("Telegram API 错误 (code: {code}): {description}")]
    Api {
        /// 错误码
        code: i32,
        /// 错误描述
        description: String,
        /// 可选的重试等待时间（秒）
        retry_after: Option<i32>,
    },

    /// 自定义业务错误
    #[error("业务错误: {0}")]
    Business(String),

    /// 其他未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
}

/// 方便从 TelegramApiError 转换成我们的错误类型
impl From<TelegramApiError> for TelegramError {
    fn from(err: TelegramApiError) -> Self {
        TelegramError::Api {
            code: err.error_code.unwrap_or(0),
            description: err.description.unwrap_or_else(|| "未知错误".to_string()),
            retry_after: err.parameters.and_then(|p| p.retry_after),
        }
    }
}

/// 结果类型别名，方便使用
pub type TelegramResult<T> = Result<T, TelegramError>;