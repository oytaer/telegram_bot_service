//! 更新相关完整类型定义
//! 严格对应官方 Update 对象

use serde::{Deserialize, Serialize};
use super::message::Message;
use super::user::User;
use super::chat::{Chat, ChatMember, ChatInviteLink, ChatJoinRequest, ChatBoostUpdated, ChatBoostRemoved};
use super::inline::InlineQuery;
use super::inline::ChosenInlineResult;
use super::inline::CallbackQuery;
use super::payment::ShippingQuery;
use super::payment::PreCheckoutQuery;
use super::payment::PaidMediaPurchased;
use crate::telegram_core::methods::advanced::business::BusinessConnection;

/// 官方 Update 对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Update {
    /// 更新唯一标识符
    pub update_id: i64,

    /// 新消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Message>,

    /// 被编辑的消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_message: Option<Message>,

    /// 频道帖子
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_post: Option<Message>,

    /// 被编辑的频道帖子
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_channel_post: Option<Message>,

    /// 商业连接（机器人连接或断开商业账号，或用户编辑了连接设置）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection: Option<BusinessConnection>,

    /// 商业消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_message: Option<Message>,

    /// 被编辑的商业消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edited_business_message: Option<Message>,

    /// 被删除的商业消息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_business_messages: Option<BusinessMessagesDeleted>,

    /// 访客消息（Bot API 10.0 新增；可通过 Message.guest_query_id 与 answerGuestQuery 方法回复）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guest_message: Option<Message>,

    /// 消息反应
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction: Option<MessageReactionUpdated>,

    /// 消息反应数量变化
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_reaction_count: Option<MessageReactionCountUpdated>,

    /// 内联查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_query: Option<InlineQuery>,

    /// 选择的内联结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chosen_inline_result: Option<ChosenInlineResult>,

    /// 回调查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_query: Option<CallbackQuery>,

    /// 物流查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_query: Option<ShippingQuery>,

    /// 预结账查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_checkout_query: Option<PreCheckoutQuery>,

    /// 已购买的付费媒体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchased_paid_media: Option<PaidMediaPurchased>,

    /// 投票
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll: Option<super::message::Poll>,

    /// 投票回答
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poll_answer: Option<PollAnswer>,

    /// 机器人自身在聊天中的成员状态更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_chat_member: Option<ChatMemberUpdated>,

    /// 聊天成员状态更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_member: Option<ChatMemberUpdated>,

    /// 加入请求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_join_request: Option<ChatJoinRequest>,

    /// 聊天提升更新
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_boost: Option<ChatBoostUpdated>,

    /// 聊天提升被移除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_chat_boost: Option<ChatBoostRemoved>,

    /// 托管机器人更新（新的托管机器人被创建，或其 token/所有者变更）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_bot: Option<ManagedBotUpdated>,

    /// 用户付费订阅状态变更（Bot API 10.2 新增）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<BotSubscriptionUpdated>,
}

/// 商业消息被删除
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    pub business_connection_id: String,
    pub chat: Chat,
    pub message_ids: Vec<i32>,
}

/// 消息反应更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    pub chat: Chat,
    pub message_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_chat: Option<Chat>,
    pub date: i64,
    pub old_reaction: Vec<ReactionType>,
    pub new_reaction: Vec<ReactionType>,
}

/// 消息反应数量更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    pub chat: Chat,
    pub message_id: i32,
    pub date: i64,
    pub reactions: Vec<ReactionCount>,
}

/// 反应类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ReactionType {
    #[serde(rename = "emoji")]
    Emoji { emoji: String },
    #[serde(rename = "custom_emoji")]
    CustomEmoji { custom_emoji_id: String },
    #[serde(rename = "paid")]
    Paid,
}

/// 反应数量
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionCount {
    #[serde(rename = "type")]
    pub type_field: ReactionType,
    pub total_count: i32,
}

/// 投票回答
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollAnswer {
    pub poll_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voter_chat: Option<Chat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    pub option_ids: Vec<i32>,
}

/// 聊天成员更新
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberUpdated {
    pub chat: Chat,
    pub from: User,
    pub date: i64,
    pub old_chat_member: ChatMember,
    pub new_chat_member: ChatMember,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_join_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub via_chat_folder_invite_link: Option<bool>,
}

/// 托管机器人更新（官方 ManagedBotUpdated）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedBotUpdated {
    /// 创建该托管机器人的用户
    pub user: User,
    /// 托管机器人信息（其 token 可通过 getManagedBotToken 方法获取）
    pub bot: User,
}

/// 机器人付费订阅状态更新（官方 BotSubscriptionUpdated，Bot API 10.2 新增）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotSubscriptionUpdated {
    /// 向机器人付费订阅的用户
    pub user: User,
    /// 机器人在发票中指定的载荷
    pub invoice_payload: String,
    /// 订阅的新状态
    pub state: BotSubscriptionState,
}

/// 机器人付费订阅状态（官方枚举：canceled、active、failed）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BotSubscriptionState {
    /// 用户取消订阅
    Canceled,
    /// 用户重新启用先前取消的订阅
    Active,
    /// 订阅付款失败
    Failed,
}