//! 论坛话题图标相关方法

use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::message::Sticker;

impl TelegramClient {
    /// 获取可用于话题图标的自定义 emoji 贴纸
    /// 对应官方方法：getForumTopicIconStickers
    pub async fn get_forum_topic_icon_stickers(&self) -> TelegramResult<Vec<Sticker>> {
        self.request_empty("getForumTopicIconStickers").await
    }
}