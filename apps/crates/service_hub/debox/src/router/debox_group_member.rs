//! DeBox群组成员管理

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::DeboxGroupMemberController;

/// 路由器
pub struct DeboxGroupMemberRouter;

impl DeboxGroupMemberRouter {
    /// 注册`DeBox群组成员管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox-group-members",
            Router::new()
                .route("/", post(DeboxGroupMemberController::create))
                .route("/list", post(DeboxGroupMemberController::list))
                .route(
                    "/{id}",
                    get(DeboxGroupMemberController::info)
                        .delete(DeboxGroupMemberController::delete),
                )
                .route("/update", put(DeboxGroupMemberController::update))
                .route(
                    "/update-status",
                    put(DeboxGroupMemberController::update_status),
                )
                .route(
                    "/sync-group-members",
                    post(DeboxGroupMemberController::sync_group_members),
                ),
        )
    }
}
