//! DeBox群组管理

use axum::{
    Router,
    routing::{get, put},
};

use crate::controller::debox_group::DeboxGroupController;

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
                    get(DeboxGroupController::info)
                        .put(DeboxGroupController::update)
                        .delete(DeboxGroupController::delete),
                )
                .route("/{id}/status", put(DeboxGroupController::update_status)),
        )
    }
}
