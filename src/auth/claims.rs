//! 三种角色的 JWT Claims

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 总后台管理员 Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdminClaims {
    /// subject = admin_user_id
    pub sub: String,
    pub username: String,
    /// 固定 role 标记，中间件校验
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

impl AdminClaims {
    pub fn new(user_id: impl Into<String>, username: impl Into<String>, expire_hours: i64) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.into(),
            username: username.into(),
            role: "admin".into(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(expire_hours)).timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }
}

/// 代理商 Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentClaims {
    pub sub: String,
    pub username: String,
    pub agent_id: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

impl AgentClaims {
    pub fn new(
        user_id: impl Into<String>,
        username: impl Into<String>,
        agent_id: impl Into<String>,
        expire_hours: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.into(),
            username: username.into(),
            agent_id: agent_id.into(),
            role: "agent".into(),
            iat: now.timestamp(),
            exp: (now + Duration::hours(expire_hours)).timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }
}

/// 租户 Claims（可绑定具体 bot 上下文，bot_id 可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantClaims {
    pub sub: String,
    pub username: String,
    /// 租户编码（用于路由到租户库）
    pub tenant_code: String,
    pub tenant_id: String,
    pub role: String,
    /// 可选：当前操作的 bot
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_id: Option<String>,
    pub exp: i64,
    pub iat: i64,
    pub jti: String,
}

impl TenantClaims {
    pub fn new(
        user_id: impl Into<String>,
        username: impl Into<String>,
        tenant_id: impl Into<String>,
        tenant_code: impl Into<String>,
        expire_hours: i64,
    ) -> Self {
        let now = Utc::now();
        Self {
            sub: user_id.into(),
            username: username.into(),
            tenant_id: tenant_id.into(),
            tenant_code: tenant_code.into(),
            role: "tenant".into(),
            bot_id: None,
            iat: now.timestamp(),
            exp: (now + Duration::hours(expire_hours)).timestamp(),
            jti: Uuid::new_v4().to_string(),
        }
    }
}
