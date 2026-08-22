//! Bot 基础信息相关方法
//! 包含 getMe、logOut、close

use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::user::User;

impl TelegramClient {
    /// 获取 Bot 自身信息
    /// 对应官方方法：getMe
    /// 无需参数
    /// 返回：User 对象
    pub async fn get_me(&self) -> TelegramResult<User> {
        self.request_empty("getMe").await
    }

    /// 从云端 Bot API 服务器登出
    /// 对应官方方法：logOut
    /// 在启动本地 Bot API 服务器前必须调用
    /// 返回：True
    pub async fn log_out(&self) -> TelegramResult<bool> {
        self.request_empty("logOut").await
    }

    /// 关闭 Bot 实例
    /// 对应官方方法：close
    /// 在把 Bot 从一个本地服务器迁移到另一个之前使用
    /// 返回：True
    pub async fn close(&self) -> TelegramResult<bool> {
        self.request_empty("close").await
    }
}