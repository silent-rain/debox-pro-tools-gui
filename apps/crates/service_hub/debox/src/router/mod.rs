//! 路由层

use axum::Router;

pub mod debox_account;
pub mod debox_group;

/// 路由器
pub struct DeboxRouter;

impl DeboxRouter {
    /// 注册`Debox管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox",
            Router::new().merge(debox_account::DeboxAccountRouter::register()), // DeBox账号管理
        )
    }
}
