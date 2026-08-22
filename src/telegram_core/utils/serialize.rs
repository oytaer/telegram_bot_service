//! 序列化相关工具

use serde::Serialize;

/// 将参数序列化为 JSON Value（方便调试）
pub fn to_json_value<T: Serialize>(value: &T) -> serde_json::Result<serde_json::Value> {
    serde_json::to_value(value)
}