//! Telegram API → 低代码组件注册中心

pub mod registry;
pub mod schema;

pub use registry::ComponentRegistry;
pub use schema::{ComponentCategory, ComponentDefinition, JsonSchema};
