//! DeBox账号关注人管理

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::DeboxAccountFollowController;

/// 路由器
pub struct DeboxAccountFollowRouter;

impl DeboxAccountFollowRouter {
    /// 注册`DeBox账号关注人管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox-accounts",
            Router::new()
                .route("/", post(DeboxAccountFollowController::create))
                .route("/list", post(DeboxAccountFollowController::list))
                .route(
                    "/{id}",
                    get(DeboxAccountFollowController::info)
                        .delete(DeboxAccountFollowController::delete),
                )
                .route("/update", put(DeboxAccountFollowController::update))
                .route(
                    "/update-status",
                    put(DeboxAccountFollowController::update_status),
                ),
        )
    }
}
