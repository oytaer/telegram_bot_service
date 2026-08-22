//! 公共 API Trait 定义
//! 方便扩展、Mock 与中间件

use async_trait::async_trait;
use crate::telegram_core::error::TelegramResult;

/// 核心 API 请求 Trait
#[async_trait]
pub trait TelegramApi {
    /// 通用请求方法
    async fn call_method<T, P>(&self, method: &str, params: &P) -> TelegramResult<T>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize + ?Sized + Send + Sync;
}