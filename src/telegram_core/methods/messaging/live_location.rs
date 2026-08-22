//! 实时位置相关方法
//! 包含 editMessageLiveLocation、stopMessageLiveLocation

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::keyboard::InlineKeyboardMarkup;

/// editMessageLiveLocation 参数
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageLiveLocationParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// 聊天 ID（选填，与 inline_message_id 二选一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// 消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// 内联消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// 纬度（必填）
    pub latitude: f64,
    /// 经度（必填）
    pub longitude: f64,
    /// 直播有效期（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i32>,
    /// 水平精度（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    /// 方向角度（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,
    /// 接近提醒半径（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i32>,
    /// 内联键盘（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// stopMessageLiveLocation 参数
#[derive(Debug, Clone, Serialize)]
pub struct StopMessageLiveLocationParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// 聊天 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// 消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// 内联消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// 内联键盘（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

impl TelegramClient {
    /// 编辑实时位置消息
    /// 对应官方方法：editMessageLiveLocation
    pub async fn edit_message_live_location(
        &self,
        params: &EditMessageLiveLocationParams,
    ) -> TelegramResult<Message> {
        self.request("editMessageLiveLocation", params).await
    }

    /// 停止实时位置更新
    /// 对应官方方法：stopMessageLiveLocation
    pub async fn stop_message_live_location(
        &self,
        params: &StopMessageLiveLocationParams,
    ) -> TelegramResult<Message> {
        self.request("stopMessageLiveLocation", params).await
    }
}