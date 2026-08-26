//! 应用配置（环境变量）

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,

    /// 平台库（总后台 + 代理商）连接串
    pub platform_database_url: String,

    /// 租户库连接模板，例如：
    /// mysql://user:pass@host:3306/tenant_{tenant_code}
    /// 独立部署时也可直接指向单一租户库
    pub tenant_database_url_template: String,

    /// 三种 JWT 密钥（可相同，建议生产环境分离）
    pub jwt_admin_secret: String,
    pub jwt_agent_secret: String,
    pub jwt_tenant_secret: String,

    /// Token 有效期（小时）
    pub jwt_expire_hours: i64,

    /// 部署模式：saas | standalone
    /// standalone：单租户独立部署，不走平台库租户路由
    pub deploy_mode: DeployMode,

    /// standalone 模式下的固定租户编码（可选）
    pub standalone_tenant_code: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DeployMode {
    Saas,
    Standalone,
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            host: std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into()),
            port: std::env::var("PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(8080),
            platform_database_url: std::env::var("PLATFORM_DATABASE_URL")
                .unwrap_or_else(|_| "mysql://root:password@127.0.0.1:3306/tg_platform".into()),
            tenant_database_url_template: std::env::var("TENANT_DATABASE_URL_TEMPLATE")
                .unwrap_or_else(|_| {
                    "mysql://root:password@127.0.0.1:3306/tg_tenant_{tenant_code}".into()
                }),
            jwt_admin_secret: std::env::var("JWT_ADMIN_SECRET")
                .unwrap_or_else(|_| "change-me-admin-secret".into()),
            jwt_agent_secret: std::env::var("JWT_AGENT_SECRET")
                .unwrap_or_else(|_| "change-me-agent-secret".into()),
            jwt_tenant_secret: std::env::var("JWT_TENANT_SECRET")
                .unwrap_or_else(|_| "change-me-tenant-secret".into()),
            jwt_expire_hours: std::env::var("JWT_EXPIRE_HOURS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(24),
            deploy_mode: match std::env::var("DEPLOY_MODE")
                .unwrap_or_else(|_| "saas".into())
                .to_lowercase()
                .as_str()
            {
                "standalone" => DeployMode::Standalone,
                _ => DeployMode::Saas,
            },
            standalone_tenant_code: std::env::var("STANDALONE_TENANT_CODE").ok(),
        })
    }

    pub fn tenant_db_url(&self, tenant_code: &str) -> String {
        self.tenant_database_url_template
            .replace("{tenant_code}", tenant_code)
    }
}
