//! 游戏相关方法
//! 包含 sendGame、setGameScore、getGameHighScores

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::user::User;
use crate::telegram_core::types::keyboard::InlineKeyboardMarkup;
use crate::telegram_core::methods::messaging::send::text::ReplyParameters;

/// sendGame 参数
#[derive(Debug, Clone, Serialize)]
pub struct SendGameParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息线程 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 游戏短名称（必填）
    pub game_short_name: String,
    /// 是否禁用通知（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// 是否保护内容（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protect_content: Option<bool>,
    /// 是否允许付费广播（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_paid_broadcast: Option<bool>,
    /// 消息效果 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_effect_id: Option<String>,
    /// 回复参数（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_parameters: Option<ReplyParameters>,
    /// 内联键盘（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

/// setGameScore 参数
#[derive(Debug, Clone, Serialize)]
pub struct SetGameScoreParams {
    /// 用户 ID（必填）
    pub user_id: i64,
    /// 分数（必填，非负）
    pub score: i32,
    /// 是否强制更新（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    /// 是否禁用自动编辑消息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_edit_message: Option<bool>,
    /// 聊天 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// 消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// 内联消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// getGameHighScores 参数
#[derive(Debug, Clone, Serialize)]
pub struct GetGameHighScoresParams {
    /// 用户 ID（必填）
    pub user_id: i64,
    /// 聊天 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// 消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// 内联消息 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
}

/// 游戏高分记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHighScore {
    /// 位置
    pub position: i32,
    /// 用户
    pub user: User,
    /// 分数
    pub score: i32,
}

impl TelegramClient {
    /// 发送游戏
    /// 对应官方方法：sendGame
    pub async fn send_game(&self, params: &SendGameParams) -> TelegramResult<Message> {
        self.request("sendGame", params).await
    }

    /// 设置游戏分数
    /// 对应官方方法：setGameScore
    pub async fn set_game_score(&self, params: &SetGameScoreParams) -> TelegramResult<Message> {
        self.request("setGameScore", params).await
    }

    /// 获取游戏高分榜
    /// 对应官方方法：getGameHighScores
    pub async fn get_game_high_scores(
        &self,
        params: &GetGameHighScoresParams,
    ) -> TelegramResult<Vec<GameHighScore>> {
        self.request("getGameHighScores", params).await
    }
}