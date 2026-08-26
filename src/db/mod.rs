//! 数据库访问层
//! - platform：总后台 + 代理商共用库
//! - tenant：每租户独立库（连接池缓存）

pub mod platform;
pub mod tenant;
