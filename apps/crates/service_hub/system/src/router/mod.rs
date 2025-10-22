//! 路由层
pub mod config;

use axum::Router;

/// 路由器
pub struct SystemRouter;

impl SystemRouter {
    /// 注册`系统管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/system",
            Router::new().merge(config::ConfigRouter::register()), // 配置管理
        )
    }
}
