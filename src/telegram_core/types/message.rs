//! 消息相关完整类型定义
//! 严格对应官方 Message 及所有嵌套类型

use serde::{Deserialize, Serialize};
use super::user::User;
use super::chat::{Chat, Location};
use super::common::MessageEntity;
use super::keyboard::InlineKeyboardMarkup;

/// 官方 Message 对象（完整字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 消息唯一 ID
    pub message_id: i32,

    /// 消息线程 ID（话题 ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,

    /// 发送者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<User>,

    /// 发送者聊天（匿名管理员或频道）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_chat: Option<Chat>,

    /// 发送者商业账号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_business_bot: Option<User>,

    /// 发送日期（Unix 时间戳）
    pub date: i64,

    /// 所属聊天
    pub chat: Chat,

    /// 转发来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_origin: Option<MessageOrigin>,

    /// 是否为自动转发
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_automatic_forward: Option<bool>,

    /// 回复的消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message: Option<Box<Message>>,

    /// 外部回复信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reply: Option<ExternalReplyInfo>,

    /// 引用文字
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote: Option<TextQuote>,

    /// 回复的故事
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_story: Option<Story>,

    /// 通过哪个 Bot 发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_bot: Option<User>,

    /// 编辑日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit_date: Option<i64>,

    /// 是否有保护内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,

    /// 是否来自离线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_offline: Option<bool>,

    /// 媒体组 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_group_id: Option<String>,

    /// 作者签名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author_signature: Option<String>,

    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,

    /// 文本实体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,

    /// 链接预览选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,

    /// 特效 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_id: Option<String>,

    /// 动画
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,

    /// 音频
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,

    /// 文档
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,

    /// 付费媒体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,

    /// 照片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,

    /// 贴纸
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,

    /// 故事
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,

    /// 视频
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,

    /// 视频笔记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,

    /// 语音
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,

    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// 标题实体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption_entities: Option<Vec<MessageEntity>>,

    /// 是否在媒体上方显示标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_caption_above_media: Option<bool>,

    /// 是否有媒体伪装
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,

    /// 联系人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,

    /// 骰子
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,

    /// 游戏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,

    /// 投票
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,

    /// 地点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,

    /// 位置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,

    /// 新成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_members: Option<Vec<User>>,

    /// 离开成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_chat_member: Option<User>,

    /// 新聊天标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_title: Option<String>,

    /// 新聊天照片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_chat_photo: Option<Vec<PhotoSize>>,

    /// 删除聊天照片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_chat_photo: Option<bool>,

    /// 群组创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_chat_created: Option<bool>,

    /// 超级群创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supergroup_chat_created: Option<bool>,

    /// 频道创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_chat_created: Option<bool>,

    /// 消息自动删除定时器变更
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_timer_changed: Option<MessageAutoDeleteTimerChanged>,

    /// 迁移到聊天 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_to_chat_id: Option<i64>,

    /// 从聊天 ID 迁移而来
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migrate_from_chat_id: Option<i64>,

    /// 置顶消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<MaybeInaccessibleMessage>>,

    /// 发票
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,

    /// 成功支付
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful_payment: Option<SuccessfulPayment>,

    /// 退款支付
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_payment: Option<RefundedPayment>,

    /// 用户共享
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users_shared: Option<UsersShared>,

    /// 聊天共享
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_shared: Option<ChatShared>,

    /// 关联网站
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_website: Option<String>,

    /// 写入访问允许
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_allowed: Option<WriteAccessAllowed>,

    /// 护照数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_data: Option<PassportData>,

    /// 临近警报
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_triggered: Option<ProximityAlertTriggered>,

    /// 提升添加
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boost_added: Option<ChatBoostAdded>,

    /// 聊天背景设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_background_set: Option<ChatBackground>,

    /// 论坛话题创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_created: Option<ForumTopicCreated>,

    /// 论坛话题编辑
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_edited: Option<ForumTopicEdited>,

    /// 论坛话题关闭
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_closed: Option<ForumTopicClosed>,

    /// 论坛话题重新打开
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forum_topic_reopened: Option<ForumTopicReopened>,

    /// 总则话题隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_hidden: Option<GeneralForumTopicHidden>,

    /// 总则话题取消隐藏
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_forum_topic_unhidden: Option<GeneralForumTopicUnhidden>,

    /// 赠与创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_created: Option<GiveawayCreated>,

    /// 赠与
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,

    /// 赠与获奖者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,

    /// 赠与完成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_completed: Option<GiveawayCompleted>,

    /// 视频聊天计划
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_scheduled: Option<VideoChatScheduled>,

    /// 视频聊天开始
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_started: Option<VideoChatStarted>,

    /// 视频聊天结束
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_ended: Option<VideoChatEnded>,

    /// 视频聊天参与者邀请
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_chat_participants_invited: Option<VideoChatParticipantsInvited>,

    /// WebApp 数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_data: Option<WebAppData>,

    /// 回复标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

// ===================== 嵌套类型完整定义 =====================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MessageOrigin {
    #[serde(rename = "user")]
    User {
        date: i64,
        sender_user: User,
    },
    #[serde(rename = "hidden_user")]
    HiddenUser {
        date: i64,
        sender_user_name: String,
    },
    #[serde(rename = "chat")]
    Chat {
        date: i64,
        sender_chat: Chat,
        #[serde(skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
    #[serde(rename = "channel")]
    Channel {
        date: i64,
        chat: Chat,
        message_id: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        author_signature: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalReplyInfo {
    pub origin: MessageOrigin,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_preview_options: Option<LinkPreviewOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<Audio>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<Document>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_media: Option<PaidMediaInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker: Option<Sticker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story: Option<Story>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video: Option<Video>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_note: Option<VideoNote>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<Voice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_media_spoiler: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dice: Option<Dice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game: Option<Game>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway: Option<Giveaway>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_winners: Option<GiveawayWinners>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Invoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<Poll>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<Venue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextQuote {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<MessageEntity>>,
    pub position: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_manual: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkPreviewOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_small_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefer_large_media: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_above_text: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Animation {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub file_unique_id: String,
    pub width: i32,
    pub height: i32,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoNote {
    pub file_id: String,
    pub file_unique_id: String,
    pub length: i32,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub file_unique_id: String,
    pub duration: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcard: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dice {
    pub emoji: String,
    pub value: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollOption {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    pub voter_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Poll {
    pub id: String,
    pub question: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question_entities: Option<Vec<MessageEntity>>,
    pub options: Vec<PollOption>,
    pub total_voter_count: i32,
    pub is_closed: bool,
    pub is_anonymous: bool,
    #[serde(rename = "type")]
    pub type_field: String,
    pub allows_multiple_answers: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_option_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_date: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

// 以下为剩余常用嵌套类型（保持完整字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub width: i32,
    pub height: i32,
    pub is_animated: bool,
    pub is_video: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_animation: Option<File>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mask_position: Option<MaskPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub needs_repainting: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_unique_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaskPosition {
    pub point: String,
    pub x_shift: f64,
    pub y_shift: f64,
    pub scale: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub chat: Chat,
    pub id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub title: String,
    pub description: String,
    pub photo: Vec<PhotoSize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_entities: Option<Vec<MessageEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation: Option<Animation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub title: String,
    pub description: String,
    pub start_parameter: String,
    pub currency: String,
    pub total_amount: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessfulPayment {
    pub currency: String,
    pub total_amount: i32,
    pub invoice_payload: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_expiration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_recurring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_recurring: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_option_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_info: Option<OrderInfo>,
    pub telegram_payment_charge_id: String,
    pub provider_payment_charge_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<ShippingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingAddress {
    pub country_code: String,
    pub state: String,
    pub city: String,
    pub street_line1: String,
    pub street_line2: String,
    pub post_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundedPayment {
    pub currency: String,
    pub total_amount: i32,
    pub invoice_payload: String,
    pub telegram_payment_charge_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_payment_charge_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersShared {
    pub request_id: i32,
    pub users: Vec<SharedUser>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedUser {
    pub user_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatShared {
    pub request_id: i32,
    pub chat_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_attachment_menu: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PassportFile {
    pub file_id: String,
    pub file_unique_id: String,
    pub file_size: i32,
    pub file_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCredentials {
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProximityAlertTriggered {
    pub traveler: User,
    pub watcher: User,
    pub distance: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostAdded {
    pub boost_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBackground {
    #[serde(rename = "type")]
    pub type_field: BackgroundType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundType {
    #[serde(rename = "fill")]
    Fill {
        fill: BackgroundFill,
        dark_theme_dimming: i32,
    },
    #[serde(rename = "wallpaper")]
    Wallpaper {
        document: Document,
        dark_theme_dimming: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_blurred: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_moving: Option<bool>,
    },
    #[serde(rename = "pattern")]
    Pattern {
        document: Document,
        fill: BackgroundFill,
        intensity: i32,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_inverted: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        is_moving: Option<bool>,
    },
    #[serde(rename = "chat_theme")]
    ChatTheme {
        theme_name: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BackgroundFill {
    #[serde(rename = "solid")]
    Solid { color: i32 },
    #[serde(rename = "gradient")]
    Gradient {
        top_color: i32,
        bottom_color: i32,
        rotation_angle: i32,
    },
    #[serde(rename = "freeform_gradient")]
    FreeformGradient { colors: Vec<i32> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopicCreated {
    pub name: String,
    pub icon_color: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopicEdited {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_custom_emoji_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopicClosed {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForumTopicReopened {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralForumTopicHidden {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralForumTopicUnhidden {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiveawayCreated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Giveaway {
    pub chats: Vec<Chat>,
    pub winners_selection_date: i64,
    pub winner_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_public_winners: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiveawayWinners {
    pub chat: Chat,
    pub giveaway_message_id: i32,
    pub winners_selection_date: i64,
    pub winner_count: i32,
    pub winners: Vec<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_chat_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_subscription_month_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_new_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub was_refunded: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiveawayCompleted {
    pub winner_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unclaimed_prize_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giveaway_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_star_giveaway: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoChatScheduled {
    pub start_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoChatStarted {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoChatEnded {
    pub duration: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoChatParticipantsInvited {
    pub users: Vec<User>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppData {
    pub data: String,
    pub button_text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    pub message_auto_delete_time: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaidMediaInfo {
    pub star_count: i32,
    pub paid_media: Vec<PaidMedia>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PaidMedia {
    #[serde(rename = "preview")]
    Preview {
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        height: Option<i32>,
        #[serde(skip_serializing_if = "Option::is_none")]
        duration: Option<i32>,
    },
    #[serde(rename = "photo")]
    Photo {
        photo: Vec<PhotoSize>,
    },
    #[serde(rename = "video")]
    Video {
        video: Video,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeInaccessibleMessage {
    Message(Box<Message>),
    InaccessibleMessage(InaccessibleMessage),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    pub chat: Chat,
    pub message_id: i32,
    pub date: i64,
}