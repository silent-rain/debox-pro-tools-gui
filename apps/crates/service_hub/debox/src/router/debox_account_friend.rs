//! DeBox账号好友管理

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::DeboxAccountFriendController;

/// 路由器
pub struct DeboxAccountFriendRouter;

impl DeboxAccountFriendRouter {
    /// 注册`DeBox账号好友管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox-account-friends",
            Router::new()
                .route("/", post(DeboxAccountFriendController::create))
                .route("/list", post(DeboxAccountFriendController::list))
                .route(
                    "/{id}",
                    get(DeboxAccountFriendController::info)
                        .delete(DeboxAccountFriendController::delete),
                )
                .route("/update", put(DeboxAccountFriendController::update))
                .route(
                    "/update-status",
                    put(DeboxAccountFriendController::update_status),
                )
                .route(
                    "/sync-friends",
                    post(DeboxAccountFriendController::sync_friends),
                )
                .route(
                    "/send-private-message-text",
                    post(DeboxAccountFriendController::send_private_message_text),
                ),
        )
    }
}
