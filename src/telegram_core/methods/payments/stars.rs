//! Telegram Stars 相关方法
//! 包含 getStarTransactions、refundStarPayment

use serde::{Deserialize, Serialize};
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;

/// Stars 交易
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTransaction {
    /// 交易唯一 ID
    pub id: String,
    /// 交易金额（负数为支出）
    pub amount: i32,
    /// 交易日期（Unix 时间戳）
    pub date: i64,
    /// 来源（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<TransactionPartner>,
    /// 接收方（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<TransactionPartner>,
}

/// 交易伙伴
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TransactionPartner {
    #[serde(rename = "user")]
    User {
        user: crate::telegram_core::types::user::User,
        #[serde(skip_serializing_if = "Option::is_none")]
        invoice_payload: Option<String>,
    },
    #[serde(rename = "fragment")]
    Fragment {},
    #[serde(rename = "telegram_ads")]
    TelegramAds {},
    #[serde(rename = "other")]
    Other {},
}

/// getStarTransactions 参数
#[derive(Debug, Clone, Serialize, Default)]
pub struct GetStarTransactionsParams {
    /// 偏移量（选填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i32>,
    /// 限制数量（选填，1-100）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
}

/// StarTransactions 返回结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StarTransactions {
    pub transactions: Vec<StarTransaction>,
}

/// refundStarPayment 参数
#[derive(Debug, Clone, Serialize)]
pub struct RefundStarPaymentParams {
    /// 用户 ID（必填）
    pub user_id: i64,
    /// Telegram 支付交易 ID（必填）
    pub telegram_payment_charge_id: String,
}

impl TelegramClient {
    /// 获取 Stars 交易记录
    /// 对应官方方法：getStarTransactions
    pub async fn get_star_transactions(
        &self,
        params: &GetStarTransactionsParams,
    ) -> TelegramResult<StarTransactions> {
        self.request("getStarTransactions", params).await
    }

    /// 退款 Stars 支付
    /// 对应官方方法：refundStarPayment
    pub async fn refund_star_payment(
        &self,
        params: &RefundStarPaymentParams,
    ) -> TelegramResult<bool> {
        self.request("refundStarPayment", params).await
    }
}