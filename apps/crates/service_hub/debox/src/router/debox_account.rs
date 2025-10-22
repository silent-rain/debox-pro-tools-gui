//! DeBox账号管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::debox_account::DeboxAccountController;

/// 路由器
pub struct DeboxAccountRouter;

impl DeboxAccountRouter {
    /// 注册`DeBox账号管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox-accounts",
            Router::new()
                .route(
                    "/",
                    get(DeboxAccountController::list).post(DeboxAccountController::create),
                )
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
