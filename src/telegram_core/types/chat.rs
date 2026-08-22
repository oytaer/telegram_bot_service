//! 聊天相关完整类型定义
//! 严格对应官方文档 Chat、ChatFullInfo、ChatPermissions、ChatMember 等

use serde::{Deserialize, Serialize};
use super::user::User;
use super::message::Message;

/// 聊天类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ChatType {
    Private,
    Group,
    Supergroup,
    Channel,
}

/// 基础聊天对象（官方 Chat）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    /// 聊天唯一标识符
    pub id: i64,
    /// 聊天类型
    #[serde(rename = "type")]
    pub type_field: ChatType,
    /// 标题（仅群组、超级群、频道）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 用户名（私聊、超级群、频道）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// 名（仅私聊）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// 姓（仅私聊）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// 是否为论坛超级群
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
}

/// 完整聊天信息（官方 ChatFullInfo，getChat 返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFullInfo {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: ChatType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<ChatPhoto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_usernames: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_custom_emoji_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_status_expiration_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_private_forwards: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_restricted_voice_and_video_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_to_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_by_request: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_message: Option<Box<Message>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ChatPermissions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slow_mode_delay: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_auto_delete_time: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_aggressive_anti_spam_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_hidden_members: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_protected_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_visible_history: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticker_set_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_set_sticker_set: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_chat_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<ChatLocation>,
}

/// 聊天照片
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatPhoto {
    /// 小尺寸照片的 file_id
    pub small_file_id: String,
    /// 小尺寸照片的唯一 file_unique_id
    pub small_file_unique_id: String,
    /// 大尺寸照片的 file_id
    pub big_file_id: String,
    /// 大尺寸照片的唯一 file_unique_id
    pub big_file_unique_id: String,
}

/// 聊天权限（官方 ChatPermissions）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_audios: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_documents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_photos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_videos: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_video_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_voice_notes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
}

/// 地理位置聊天
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatLocation {
    pub location: Location,
    pub address: String,
}

/// 位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<i32>,
}

/// 聊天成员（官方 ChatMember 联合类型）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ChatMember {
    #[serde(rename = "creator")]
    Owner(ChatMemberOwner),
    #[serde(rename = "administrator")]
    Administrator(ChatMemberAdministrator),
    #[serde(rename = "member")]
    Member(ChatMemberMember),
    #[serde(rename = "restricted")]
    Restricted(ChatMemberRestricted),
    #[serde(rename = "left")]
    Left(ChatMemberLeft),
    #[serde(rename = "kicked")]
    Banned(ChatMemberBanned),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberOwner {
    pub user: User,
    pub is_anonymous: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberAdministrator {
    pub user: User,
    pub can_be_edited: bool,
    pub is_anonymous: bool,
    pub can_manage_chat: bool,
    pub can_delete_messages: bool,
    pub can_manage_video_chats: bool,
    pub can_restrict_members: bool,
    pub can_promote_members: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_manage_topics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberMember {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberRestricted {
    pub user: User,
    pub is_member: bool,
    pub can_send_messages: bool,
    pub can_send_audios: bool,
    pub can_send_documents: bool,
    pub can_send_photos: bool,
    pub can_send_videos: bool,
    pub can_send_video_notes: bool,
    pub can_send_voice_notes: bool,
    pub can_send_polls: bool,
    pub can_send_other_messages: bool,
    pub can_add_web_page_previews: bool,
    pub can_change_info: bool,
    pub can_invite_users: bool,
    pub can_pin_messages: bool,
    pub can_manage_topics: bool,
    pub until_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberLeft {
    pub user: User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMemberBanned {
    pub user: User,
    pub until_date: i64,
}

/// 聊天邀请链接（官方 ChatInviteLink，含 Bot API 9.x 订阅字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatInviteLink {
    /// 邀请链接（若由其他管理员创建，链接第二部分会被替换为 "…"）
    pub invite_link: String,
    /// 邀请链接的创建者
    pub creator: User,
    /// 是否需要管理员审批通过此链接加入的请求
    pub creates_join_request: bool,
    /// 是否为主邀请链接
    pub is_primary: bool,
    /// 是否已被撤销
    pub is_revoked: bool,
    /// 邀请链接名称（选填，0-32 字符）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 链接过期时间（选填，Unix 时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_date: Option<i64>,
    /// 通过此链接可加入的最大成员数（选填，1-99999）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_limit: Option<i32>,
    /// 使用此链接产生的待处理加入请求数量（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_join_request_count: Option<i32>,
    /// 订阅在下次付款前保持激活的秒数（选填，Bot API 9.x 新增）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i32>,
    /// 用户通过此链接订阅聊天须支付的 Telegram Stars 数量（选填，Bot API 9.x 新增）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_price: Option<i32>,
}

/// 加入请求（官方 ChatJoinRequest）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatJoinRequest {
    /// 请求发送到的聊天
    pub chat: Chat,
    /// 发送加入请求的用户
    pub from: User,
    /// 与发送请求用户的私聊 ID（可能超过 32 位，需用 64 位整数安全存储；
    /// 机器人可在请求被处理前使用此 ID 发送消息 5 分钟）
    pub user_chat_id: i64,
    /// 请求发送时间（Unix 时间戳）
    pub date: i64,
    /// 用户简介（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// 用户发送加入请求所使用的聊天邀请链接（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invite_link: Option<ChatInviteLink>,
    /// 加入请求查询唯一标识符（选填，Bot API 10.1 新增）；
    /// 仅当机器人被指派处理加入请求时存在，
    /// 存在时机器人必须在 10 秒内调用 sendChatJoinRequestWebApp 或 answerChatJoinRequestQuery
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_id: Option<String>,
}

/// 聊天提升（官方 ChatBoost）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoost {
    /// 提升唯一标识符
    pub boost_id: String,
    /// 提升添加时间（Unix 时间戳）
    pub add_date: i64,
    /// 提升自动过期时间（Unix 时间戳），除非提升者更改过期时间
    pub expiration_date: i64,
    /// 提升来源
    pub source: ChatBoostSource,
}

/// 提升来源（官方 ChatBoostSource 联合类型，按 source 字段区分）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "source")]
pub enum ChatBoostSource {
    /// 来自 Telegram Premium 订阅的提升
    #[serde(rename = "premium")]
    Premium(ChatBoostSourcePremium),
    /// 来自 Telegram Premium 礼品码的提升
    #[serde(rename = "gift_code")]
    GiftCode(ChatBoostSourceGiftCode),
    /// 来自 Telegram Premium 或 Telegram Stars 抽奖的提升
    #[serde(rename = "giveaway")]
    Giveaway(ChatBoostSourceGiveaway),
}

/// Premium 订阅提升来源（官方 ChatBoostSourcePremium）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSourcePremium {
    /// 提升该聊天的用户
    pub user: User,
}

/// 礼品码提升来源（官方 ChatBoostSourceGiftCode）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSourceGiftCode {
    /// 为其创建礼品码的用户
    pub user: User,
}

/// 抽奖提升来源（官方 ChatBoostSourceGiveaway）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostSourceGiveaway {
    /// 聊天中包含抽奖的消息 ID（消息可能已被删除；若消息尚未发送则可能为 0）
    pub giveaway_message_id: i32,
    /// 抽奖中获奖的用户（选填，仅适用于 Telegram Premium 抽奖）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
    /// 在抽奖获胜者之间分配的 Telegram Stars 数量（选填，Bot API 8.0 新增，仅适用于 Stars 抽奖）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prize_star_count: Option<i32>,
    /// 抽奖已完成但无用户赢得奖品时为 true（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unclaimed: Option<bool>,
}

/// 聊天提升更新（官方 ChatBoostUpdated）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    /// 被提升的聊天
    pub chat: Chat,
    /// 提升信息
    pub boost: ChatBoost,
}

/// 聊天提升被移除（官方 ChatBoostRemoved）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    /// 被提升的聊天
    pub chat: Chat,
    /// 被移除提升的唯一标识符
    pub boost_id: String,
    /// 提升被移除的时间（Unix 时间戳）
    pub remove_date: i64,
    /// 被移除提升的来源
    pub source: ChatBoostSource,
}