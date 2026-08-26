//! 组件注册表：汇聚全部 telegram_core API 为可拖拽组件

use once_cell::sync::Lazy;
use std::collections::HashMap;

use super::schema::{ComponentCategory, ComponentDefinition, JsonSchema};

pub struct ComponentRegistry {
    by_id: HashMap<String, ComponentDefinition>,
}

impl ComponentRegistry {
    pub fn global() -> &'static ComponentRegistry {
        &GLOBAL_REGISTRY
    }

    pub fn get(&self, id: &str) -> Option<&ComponentDefinition> {
        self.by_id.get(id)
    }

    pub fn list(&self) -> Vec<&ComponentDefinition> {
        let mut v: Vec<_> = self.by_id.values().collect();
        v.sort_by(|a, b| a.id.cmp(&b.id));
        v
    }

    pub fn list_by_category(&self, cat: ComponentCategory) -> Vec<&ComponentDefinition> {
        self.by_id
            .values()
            .filter(|c| c.category == cat)
            .collect()
    }
}

fn def(
    id: &str,
    api: &str,
    title: &str,
    desc: &str,
    cat: ComponentCategory,
    required: &[&str],
) -> ComponentDefinition {
    ComponentDefinition {
        id: id.into(),
        api_method: api.into(),
        title: title.into(),
        description: desc.into(),
        category: cat,
        icon: format!("api/{api}"),
        input_schema: JsonSchema {
            schema_type: "object".into(),
            properties: None,
            required: Some(required.iter().map(|s| s.to_string()).collect()),
            description: Some(desc.into()),
        },
        canvas_visible: true,
    }
}

fn build_registry() -> ComponentRegistry {
    let mut map = HashMap::new();
    let mut add = |c: ComponentDefinition| {
        map.insert(c.id.clone(), c);
    };

    // —— Core ——
    add(def("telegram.get_me", "getMe", "获取 Bot 信息", "getMe", ComponentCategory::Core, &[]));
    add(def("telegram.get_updates", "getUpdates", "获取更新", "长轮询获取更新", ComponentCategory::Core, &[]));
    add(def("telegram.set_webhook", "setWebhook", "设置 Webhook", "setWebhook", ComponentCategory::Core, &["url"]));
    add(def("telegram.delete_webhook", "deleteWebhook", "删除 Webhook", "deleteWebhook", ComponentCategory::Core, &[]));
    add(def("telegram.set_my_commands", "setMyCommands", "设置命令菜单", "setMyCommands", ComponentCategory::Core, &["commands"]));
    add(def("telegram.get_my_commands", "getMyCommands", "获取命令菜单", "getMyCommands", ComponentCategory::Core, &[]));

    // —— Messaging ——
    add(def("telegram.send_message", "sendMessage", "发送文本", "发送文本消息", ComponentCategory::Messaging, &["chat_id", "text"]));
    add(def("telegram.send_photo", "sendPhoto", "发送图片", "sendPhoto", ComponentCategory::Messaging, &["chat_id", "photo"]));
    add(def("telegram.send_document", "sendDocument", "发送文档", "sendDocument", ComponentCategory::Messaging, &["chat_id", "document"]));
    add(def("telegram.send_video", "sendVideo", "发送视频", "sendVideo", ComponentCategory::Messaging, &["chat_id", "video"]));
    add(def("telegram.send_audio", "sendAudio", "发送音频", "sendAudio", ComponentCategory::Messaging, &["chat_id", "audio"]));
    add(def("telegram.send_voice", "sendVoice", "发送语音", "sendVoice", ComponentCategory::Messaging, &["chat_id", "voice"]));
    add(def("telegram.send_sticker", "sendSticker", "发送贴纸", "sendSticker", ComponentCategory::Messaging, &["chat_id", "sticker"]));
    add(def("telegram.send_media_group", "sendMediaGroup", "发送媒体组", "相册", ComponentCategory::Messaging, &["chat_id", "media"]));
    add(def("telegram.send_location", "sendLocation", "发送位置", "sendLocation", ComponentCategory::Messaging, &["chat_id", "latitude", "longitude"]));
    add(def("telegram.send_poll", "sendPoll", "发送投票", "sendPoll", ComponentCategory::Messaging, &["chat_id", "question", "options"]));
    add(def("telegram.send_dice", "sendDice", "发送骰子", "sendDice", ComponentCategory::Messaging, &["chat_id"]));
    add(def("telegram.edit_message_text", "editMessageText", "编辑文本", "editMessageText", ComponentCategory::Messaging, &["text"]));
    add(def("telegram.delete_message", "deleteMessage", "删除消息", "deleteMessage", ComponentCategory::Messaging, &["chat_id", "message_id"]));
    add(def("telegram.forward_message", "forwardMessage", "转发消息", "forwardMessage", ComponentCategory::Messaging, &["chat_id", "from_chat_id", "message_id"]));
    add(def("telegram.copy_message", "copyMessage", "复制消息", "copyMessage", ComponentCategory::Messaging, &["chat_id", "from_chat_id", "message_id"]));
    add(def("telegram.stop_poll", "stopPoll", "停止投票", "stopPoll", ComponentCategory::Messaging, &["chat_id", "message_id"]));

    // —— Chat ——
    add(def("telegram.ban_chat_member", "banChatMember", "封禁成员", "banChatMember", ComponentCategory::Chat, &["chat_id", "user_id"]));
    add(def("telegram.unban_chat_member", "unbanChatMember", "解封成员", "unbanChatMember", ComponentCategory::Chat, &["chat_id", "user_id"]));
    add(def("telegram.restrict_chat_member", "restrictChatMember", "限制成员", "restrictChatMember", ComponentCategory::Chat, &["chat_id", "user_id", "permissions"]));
    add(def("telegram.promote_chat_member", "promoteChatMember", "提升管理员", "promoteChatMember", ComponentCategory::Chat, &["chat_id", "user_id"]));
    add(def("telegram.get_chat", "getChat", "获取聊天信息", "getChat", ComponentCategory::Chat, &["chat_id"]));
    add(def("telegram.get_chat_member", "getChatMember", "获取成员信息", "getChatMember", ComponentCategory::Chat, &["chat_id", "user_id"]));
    add(def("telegram.pin_chat_message", "pinChatMessage", "置顶消息", "pinChatMessage", ComponentCategory::Chat, &["chat_id", "message_id"]));
    add(def("telegram.leave_chat", "leaveChat", "退出聊天", "leaveChat", ComponentCategory::Chat, &["chat_id"]));

    // —— Inline ——
    add(def("telegram.answer_callback_query", "answerCallbackQuery", "应答回调查询", "answerCallbackQuery", ComponentCategory::Inline, &["callback_query_id"]));
    add(def("telegram.answer_inline_query", "answerInlineQuery", "应答内联查询", "answerInlineQuery", ComponentCategory::Inline, &["inline_query_id", "results"]));

    ComponentRegistry { by_id: map }
}

static GLOBAL_REGISTRY: Lazy<ComponentRegistry> = Lazy::new(build_registry);
