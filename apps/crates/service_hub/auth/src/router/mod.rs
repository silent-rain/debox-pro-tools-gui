//! 路由层

use axum::Router;
pub mod login;
pub mod logout;
pub mod register;

/// 路由器
pub struct AuthRouter;

impl AuthRouter {
    /// 注册`认证管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/auth",
            Router::new()
                .merge(login::LoginRouter::register()) // 登陆
                .merge(logout::LogoutRouter::register()) // 登出
                .merge(register::RegisterRouter::register()), // 注册用户
        )
    }
}
