//! 富文本消息相关类型定义（Bot API 10.x）

use serde::Serialize;
use super::common::MessageEntity;

/// 富文本消息输入（官方 InputRichMessage）
#[derive(Debug, Clone, Serialize)]
pub struct InputRichMessage {
    /// 使用 markdown / html 时的媒体（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media: Option<Vec<InputRichMessageMedia>>,
    /// 使用 block 结构时的内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<InputRichBlock>>,
}

/// 富文本媒体
#[derive(Debug, Clone, Serialize)]
pub struct InputRichMessageMedia {
    /// 媒体类型
    #[serde(rename = "type")]
    pub type_field: String,
    /// 媒体文件
    pub media: String,
}

/// 富文本块（简化常用类型）
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum InputRichBlock {
    #[serde(rename = "paragraph")]
    Paragraph {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        entities: Option<Vec<MessageEntity>>,
    },
    #[serde(rename = "section_heading")]
    SectionHeading {
        text: String,
    },
    #[serde(rename = "divider")]
    Divider {},
    #[serde(rename = "preformatted")]
    Preformatted {
        text: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
    },
}