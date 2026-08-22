//! 解析相关工具

/// 安全解析 ChatId
pub fn parse_chat_id(input: &str) -> Option<crate::telegram_core::types::common::ChatId> {
    if let Ok(id) = input.parse::<i64>() {
        Some(crate::telegram_core::types::common::ChatId::Id(id))
    } else if input.starts_with('@') {
        Some(crate::telegram_core::types::common::ChatId::Username(input.to_string()))
    } else {
        None
    }
}