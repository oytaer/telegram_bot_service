//! Telegram Bot API 核心客户端
//! 负责发送所有 HTTP 请求，统一处理 Token、序列化、错误解析

use crate::telegram_core::error::{TelegramError, TelegramResult, TelegramApiError};
use reqwest::Client; // 异步 HTTP 客户端
use serde::de::DeserializeOwned; // 反序列化 trait
use serde::Serialize; // 序列化 trait
use std::time::Duration; // 超时控制

/// Telegram Bot 客户端
#[derive(Clone)]
pub struct TelegramClient {
    /// Bot Token（从 BotFather 获取）
    token: String,
    /// 底层 HTTP 客户端
    http: Client,
    /// API 基础地址（默认官方地址，支持本地 Bot API）
    base_url: String,
}

impl TelegramClient {
    /// 创建一个新的客户端实例
    ///
    /// # 参数
    /// - `token`: Bot Token
    pub fn new(token: impl Into<String>) -> Self {
        // 创建带超时的 HTTP 客户端
        let http = Client::builder()
            .timeout(Duration::from_secs(30)) // 默认 30 秒超时
            .build()
            .expect("创建 HTTP 客户端失败");

        Self {
            token: token.into(),
            http,
            base_url: "https://api.telegram.org".to_string(), // 官方地址
        }
    }

    /// 使用自定义 base_url（用于本地 Bot API Server）
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }

    /// 通用请求方法（所有 API 最终都调用这个）
    ///
    /// # 类型参数
    /// - `T`: 成功时返回的数据类型
    /// - `P`: 请求参数类型（必须实现 Serialize）
    pub async fn request<T, P>(&self, method: &str, params: &P) -> TelegramResult<T>
    where
        T: DeserializeOwned, // 返回值必须能反序列化
        P: Serialize + ?Sized, // 参数必须能序列化
    {
        // 拼接完整 URL：https://api.telegram.org/bot<token>/METHOD_NAME
        let url = format!("{}/bot{}/{}", self.base_url, self.token, method);

        // 发送 POST 请求（官方推荐使用 POST）
        let response = self
            .http
            .post(&url)
            .json(params) // 自动序列化为 JSON
            .send()
            .await?; // 网络错误会转换成 TelegramError::Http

        // 检查 HTTP 状态码
        if !response.status().is_success() {
            // 尝试解析 Telegram 的错误格式
            let api_err: TelegramApiError = response.json().await.unwrap_or(TelegramApiError {
                ok: false,
                description: Some("HTTP 请求失败".to_string()),
                error_code: None,
                parameters: None,
            });
            return Err(api_err.into());
        }

        // 解析成功响应
        // Telegram 成功时返回 { "ok": true, "result": ... }
        #[derive(serde::Deserialize)]
        struct ApiResponse<T> {
            ok: bool,
            result: Option<T>,
            description: Option<String>,
            error_code: Option<i32>,
        }

        let api_resp: ApiResponse<T> = response.json().await?;

        if api_resp.ok {
            // 成功，返回 result
            api_resp
                .result
                .ok_or_else(|| TelegramError::Business("result 字段为空".to_string()))
        } else {
            // 失败，转成我们的错误类型
            Err(TelegramError::Api {
                code: api_resp.error_code.unwrap_or(0),
                description: api_resp.description.unwrap_or_else(|| "未知错误".to_string()),
                retry_after: None,
            })
        }
    }

    /// 无参数请求（例如 getMe）
    pub async fn request_empty<T>(&self, method: &str) -> TelegramResult<T>
    where
        T: DeserializeOwned,
    {
        // 传一个空的 JSON 对象
        self.request(method, &serde_json::json!({})).await
    }
}