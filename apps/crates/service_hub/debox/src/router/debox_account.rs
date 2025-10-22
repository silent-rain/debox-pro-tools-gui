//! DeBox账号管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::config::DeboxAccountController;

/// 路由器
pub struct DeboxAccountRouter;

impl DeboxAccountRouter {
    /// 注册`配置管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/configs",
            Router::new()
                .route(
                    "/",
                    get(DeboxAccountController::list).post(DeboxAccountController::create),
                )
                .route("/tree", get(DeboxAccountController::tree))
                .route(
                    "/{id}",
                    get(DeboxAccountController::info)
                        .put(DeboxAccountController::update)
                        .delete(DeboxAccountController::delete),
                )
                .route("/{id}/status", put(DeboxAccountController::update_status)),
        )
    }
}
