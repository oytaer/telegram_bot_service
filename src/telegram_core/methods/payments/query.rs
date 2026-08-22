//! 支付查询相关方法
//! 包含 answerShippingQuery、answerPreCheckoutQuery

use serde::Serialize;
use crate::telegram_core::client::TelegramClient;
use crate::telegram_core::error::TelegramResult;
use super::invoice::LabeledPrice;

/// 物流选项
#[derive(Debug, Clone, Serialize)]
pub struct ShippingOption {
    /// 物流选项 ID
    pub id: String,
    /// 标题
    pub title: String,
    /// 价格明细
    pub prices: Vec<LabeledPrice>,
}

/// answerShippingQuery 参数
#[derive(Debug, Clone, Serialize)]
pub struct AnswerShippingQueryParams {
    /// 物流查询唯一 ID（必填）
    pub shipping_query_id: String,
    /// 是否成功（必填）
    pub ok: bool,
    /// 可用物流选项（ok 为 true 时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_options: Option<Vec<ShippingOption>>,
    /// 错误信息（ok 为 false 时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

/// answerPreCheckoutQuery 参数
#[derive(Debug, Clone, Serialize)]
pub struct AnswerPreCheckoutQueryParams {
    /// 预结账查询唯一 ID（必填）
    pub pre_checkout_query_id: String,
    /// 是否确认支付（必填）
    pub ok: bool,
    /// 错误信息（ok 为 false 时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

impl TelegramClient {
    /// 回答物流查询
    /// 对应官方方法：answerShippingQuery
    pub async fn answer_shipping_query(
        &self,
        params: &AnswerShippingQueryParams,
    ) -> TelegramResult<bool> {
        self.request("answerShippingQuery", params).await
    }

    /// 回答预结账查询
    /// 对应官方方法：answerPreCheckoutQuery
    pub async fn answer_pre_checkout_query(
        &self,
        params: &AnswerPreCheckoutQueryParams,
    ) -> TelegramResult<bool> {
        self.request("answerPreCheckoutQuery", params).await
    }
}