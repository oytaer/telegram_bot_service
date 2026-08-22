//! 键盘相关完整类型定义
//! 严格对应官方 InlineKeyboardMarkup、ReplyKeyboardMarkup 等所有类型

use serde::{Deserialize, Serialize};

/// 回复标记联合类型（官方 ReplyMarkup）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    /// 内联键盘
    InlineKeyboard(InlineKeyboardMarkup),
    /// 自定义回复键盘
    ReplyKeyboard(ReplyKeyboardMarkup),
    /// 移除回复键盘
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    /// 强制回复
    ForceReply(ForceReply),
}

/// 内联键盘标记
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardMarkup {
    /// 按钮行数组
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

/// 内联键盘按钮（完整官方字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineKeyboardButton {
    /// 按钮上显示的文字（必填）
    pub text: String,

    /// HTTP 或 tg:// 链接（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// 回调数据（选填，1-64 字节）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_data: Option<String>,

    /// Web App 信息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,

    /// 登录 URL（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_url: Option<LoginUrl>,

    /// 切换内联查询（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query: Option<String>,

    /// 当前聊天切换内联查询（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_current_chat: Option<String>,

    /// 切换内联查询选择的聊天（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_inline_query_chosen_chat: Option<SwitchInlineQueryChosenChat>,

    /// 复制文本（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_text: Option<CopyTextButton>,

    /// 回调游戏（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_game: Option<CallbackGame>,

    /// 是否为付费按钮（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay: Option<bool>,
}

/// Web App 信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebAppInfo {
    /// Web App 的 HTTPS URL
    pub url: String,
}

/// 登录 URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUrl {
    /// 登录链接
    pub url: String,
    /// 按钮上方提示文字（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_text: Option<String>,
    /// 授权域名的用户名（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_username: Option<String>,
    /// 是否请求写入权限（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_write_access: Option<bool>,
}

/// 切换内联查询到指定聊天
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    /// 查询前缀（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// 是否允许私聊（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_user_chats: Option<bool>,
    /// 是否允许机器人聊天（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_bot_chats: Option<bool>,
    /// 是否允许群组（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_group_chats: Option<bool>,
    /// 是否允许频道（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_channel_chats: Option<bool>,
}

/// 复制文本按钮
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyTextButton {
    /// 要复制的文本（1-256 字符）
    pub text: String,
}

/// 回调游戏（空对象，仅作占位）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallbackGame {}

/// 自定义回复键盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardMarkup {
    /// 按钮行数组
    pub keyboard: Vec<Vec<KeyboardButton>>,

    /// 是否为持久键盘（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_persistent: Option<bool>,

    /// 是否自适应高度（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_keyboard: Option<bool>,

    /// 是否一次性键盘（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_time_keyboard: Option<bool>,

    /// 占位提示文字（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,

    /// 是否只对特定用户显示（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// 自定义键盘按钮（完整官方字段）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// 按钮文字（必填）
    pub text: String,

    /// 请求用户信息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_users: Option<KeyboardButtonRequestUsers>,

    /// 请求聊天信息（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_chat: Option<KeyboardButtonRequestChat>,

    /// 请求联系人（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    /// 请求位置（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,

    /// 请求投票（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_poll: Option<KeyboardButtonPollType>,

    /// Web App（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_app: Option<WebAppInfo>,
}

/// 请求用户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonRequestUsers {
    /// 请求标识符
    pub request_id: i32,
    /// 是否请求机器人（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_bot: Option<bool>,
    /// 是否请求 Premium 用户（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_is_premium: Option<bool>,
    /// 最大用户数量（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_quantity: Option<i32>,
    /// 是否请求姓名（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_name: Option<bool>,
    /// 是否请求用户名（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// 是否请求照片（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// 请求聊天
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonRequestChat {
    /// 请求标识符
    pub request_id: i32,
    /// 是否请求频道（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_channel: Option<bool>,
    /// 是否请求论坛超级群（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,
    /// 是否请求有用户名的聊天（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,
    /// 是否请求已创建的聊天（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,
    /// 用户管理员权限（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,
    /// 机器人管理员权限（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,
    /// 是否请求机器人成员（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_is_member: Option<bool>,
    /// 是否请求标题（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_title: Option<bool>,
    /// 是否请求用户名（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_username: Option<bool>,
    /// 是否请求照片（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_photo: Option<bool>,
}

/// 聊天管理员权限（用于请求聊天时）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatAdministratorRights {
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
    pub can_post_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_stories: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_stories: Option<bool>,
}

/// 请求投票类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyboardButtonPollType {
    /// 投票类型（quiz 或 regular，选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

/// 移除回复键盘
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyKeyboardRemove {
    /// 必须为 true
    pub remove_keyboard: bool,
    /// 是否只对特定用户移除（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}

/// 强制回复
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForceReply {
    /// 必须为 true
    pub force_reply: bool,
    /// 输入框占位文字（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_field_placeholder: Option<String>,
    /// 是否只对特定用户生效（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selective: Option<bool>,
}