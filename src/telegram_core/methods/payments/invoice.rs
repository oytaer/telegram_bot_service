//! 发票相关方法
//! 包含 sendInvoice、createInvoiceLink

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use crate::telegram_core::types::common::ChatId;
use crate::telegram_core::types::message::Message;
use crate::telegram_core::types::keyboard::InlineKeyboardMarkup;
use crate::telegram_core::methods::messaging::send::text::ReplyParameters;

/// 标记价格
#[derive(Debug, Clone, Serialize)]
pub struct LabeledPrice {
    /// 标签
    pub label: String,
    /// 金额（最小货币单位）
    pub amount: i32,
}

/// sendInvoice 参数
#[derive(Debug, Clone, Serialize)]
pub struct SendInvoiceParams {
    /// 目标聊天 ID（必填）
    pub chat_id: ChatId,
    /// 消息线程 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_thread_id: Option<i32>,
    /// 产品名称（必填，1-32字符）
    pub title: String,
    /// 产品描述（必填，1-255字符）
    pub description: String,
    /// 载荷（必填，1-128字节）
    pub payload: String,
    /// 支付提供商代币（选填，Stars 支付时可为空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// 货币代码（必填，三位）
    pub currency: String,
    /// 价格明细（必填）
    pub prices: Vec<LabeledPrice>,
    /// 最大小费金额（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i32>,
    /// 建议小费金额（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i32>>,
    /// 开始参数（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_parameter: Option<String>,
    /// 提供者数据（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// 照片 URL（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// 照片尺寸（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i32>,
    /// 照片宽度（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,
    /// 照片高度（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,
    /// 是否需要姓名（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// 是否需要电话（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// 是否需要邮箱（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// 是否需要收货地址（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// 是否发送电话给提供者（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// 是否发送邮箱给提供者（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// 是否灵活价格（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
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

/// createInvoiceLink 参数
#[derive(Debug, Clone, Serialize)]
pub struct CreateInvoiceLinkParams {
    /// 业务连接 ID（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_connection_id: Option<String>,
    /// 产品名称（必填）
    pub title: String,
    /// 产品描述（必填）
    pub description: String,
    /// 载荷（必填）
    pub payload: String,
    /// 支付提供商代币（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_token: Option<String>,
    /// 货币代码（必填）
    pub currency: String,
    /// 价格明细（必填）
    pub prices: Vec<LabeledPrice>,
    /// 订阅周期天数（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_period: Option<i32>,
    /// 最大小费金额（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tip_amount: Option<i32>,
    /// 建议小费金额（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_tip_amounts: Option<Vec<i32>>,
    /// 提供者数据（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_data: Option<String>,
    /// 照片 URL（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_url: Option<String>,
    /// 照片尺寸（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_size: Option<i32>,
    /// 照片宽度（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,
    /// 照片高度（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,
    /// 是否需要姓名（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_name: Option<bool>,
    /// 是否需要电话（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_phone_number: Option<bool>,
    /// 是否需要邮箱（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_email: Option<bool>,
    /// 是否需要收货地址（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_shipping_address: Option<bool>,
    /// 是否发送电话给提供者（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_phone_number_to_provider: Option<bool>,
    /// 是否发送邮箱给提供者（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email_to_provider: Option<bool>,
    /// 是否灵活价格（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_flexible: Option<bool>,
}

impl TelegramClient {
    /// 发送发票
    /// 对应官方方法：sendInvoice
    pub async fn send_invoice(&self, params: &SendInvoiceParams) -> TelegramResult<Message> {
        self.request("sendInvoice", params).await
    }

    /// 创建发票链接
    /// 对应官方方法：createInvoiceLink
    pub async fn create_invoice_link(
        &self,
        params: &CreateInvoiceLinkParams,
    ) -> TelegramResult<String> {
        self.request("createInvoiceLink", params).await
    }
}