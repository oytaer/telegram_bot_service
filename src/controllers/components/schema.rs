//! 组件元数据定义

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ComponentCategory {
    Core,
    Messaging,
    Chat,
    Forum,
    Inline,
    Stickers,
    Payments,
    Games,
    Files,
    Advanced,
    Business,
    Gifts,
    Stories,
}

/// 简化版 JSON Schema（前端属性面板用）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchema {
    #[serde(rename = "type")]
    pub schema_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentDefinition {
    /// 全局唯一，如 telegram.send_message
    pub id: String,
    /// 官方 API 方法名，如 sendMessage
    pub api_method: String,
    pub title: String,
    pub description: String,
    pub category: ComponentCategory,
    pub icon: String,
    pub input_schema: JsonSchema,
    /// 是否适合出现在可视化画布上
    pub canvas_visible: bool,
}
