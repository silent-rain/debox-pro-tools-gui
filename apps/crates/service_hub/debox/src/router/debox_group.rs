//! DeBox群组管理

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::DeboxGroupController;

/// 路由器
pub struct DeboxGroupRouter;

impl DeboxGroupRouter {
    /// 注册`DeBox群组管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox-groups",
            Router::new()
                .route(
                    "/",
                    get(DeboxGroupController::list).post(DeboxGroupController::create),
                )
                .route(
                    "/{id}",
                    get(DeboxGroupController::info).delete(DeboxGroupController::delete),
                )
                .route("/update", put(DeboxGroupController::update))
                .route("/update-status", put(DeboxGroupController::update_status))
                .route("/sync-groups", post(DeboxGroupController::sync_groups)),
        )
    }
}
