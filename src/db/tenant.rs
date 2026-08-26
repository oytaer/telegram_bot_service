//! 租户库管理器
//! SaaS：按 tenant_code 动态连接并缓存 Pool
//! Standalone：使用固定租户库

use dashmap::DashMap;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;

use crate::config::{AppConfig, DeployMode};

#[derive(Clone)]
pub struct TenantDbManager {
    config: AppConfig,
    pools: Arc<DashMap<String, MySqlPool>>,
}

impl TenantDbManager {
    pub fn new(config: AppConfig) -> Self {
        Self {
            config,
            pools: Arc::new(DashMap::new()),
        }
    }

    /// 获取租户库连接池（自动缓存）
    pub async fn pool_for(&self, tenant_code: &str) -> anyhow::Result<MySqlPool> {
        if let Some(p) = self.pools.get(tenant_code) {
            return Ok(p.clone());
        }

        let url = match self.config.deploy_mode {
            DeployMode::Standalone => {
                // 独立部署：模板即真实库，或使用 standalone_tenant_code
                let code = self
                    .config
                    .standalone_tenant_code
                    .as_deref()
                    .unwrap_or(tenant_code);
                self.config.tenant_db_url(code)
            }
            DeployMode::Saas => self.config.tenant_db_url(tenant_code),
        };

        let pool = MySqlPoolOptions::new()
            .max_connections(10)
            .connect(&url)
            .await?;

        self.pools.insert(tenant_code.to_string(), pool.clone());
        Ok(pool)
    }

    /// 总后台：按编码主动打开任意租户库
    pub async fn admin_open_tenant(&self, tenant_code: &str) -> anyhow::Result<MySqlPool> {
        self.pool_for(tenant_code).await
    }

    pub fn invalidate(&self, tenant_code: &str) {
        self.pools.remove(tenant_code);
    }
}
