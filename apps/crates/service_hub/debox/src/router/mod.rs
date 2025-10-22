//! 路由层

use axum::Router;

pub mod api_operation;
pub mod debox_account;
pub mod system_log;
pub mod web_log;

/// 路由器
pub struct LogRouter;

impl LogRouter {
    /// 注册`Debox管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox",
            Router::new().merge(debox_account::DeboxAccountRouter::register()), // DeBox账号管理
        )
    }
}
