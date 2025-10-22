//! 路由层

pub mod phone;
pub mod role;
pub mod user_base;
pub mod user_role_rel;

use axum::Router;

/// 路由器
pub struct UserRouter;

impl UserRouter {
    /// 注册`用户管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/user",
            Router::new()
                .merge(role::RoleRouter::register()) // 角色管理
                .merge(user_role_rel::UserRoleRelRouter::register()) // 用户角色关系管理
                .merge(user_base::UserBaseRouter::register()) // 用户信息管理
                .merge(phone::PhoneRouter::register()), // 用户手机号管理
        )
    }
}
