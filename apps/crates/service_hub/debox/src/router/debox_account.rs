//! DeBox账号管理

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::DeboxAccountController;

/// 路由器
pub struct DeboxAccountRouter;

impl DeboxAccountRouter {
    /// 注册`DeBox账号管理`路由
    pub fn register() -> Router {
        Router::new().nest(
            "/debox-accounts",
            Router::new()
                .route("/", post(DeboxAccountController::create))
                .route("/list", post(DeboxAccountController::list))
                .route(
                    "/{id}",
                    get(DeboxAccountController::info).delete(DeboxAccountController::delete),
                )
                .route("/update", put(DeboxAccountController::update))
                .route("/update-status", put(DeboxAccountController::update_status))
                .route(
                    "/update-all-accounts-info",
                    put(DeboxAccountController::update_all_accounts_info),
                )
                .route(
                    "/update-account-info",
                    put(DeboxAccountController::update_account_info),
                )
                .route(
                    "/{id}/download-config",
                    get(DeboxAccountController::download_config_file),
                )
                .route(
                    "/upload-config",
                    post(DeboxAccountController::upload_config_file),
                )
                .route(
                    "/upload-configs",
                    post(DeboxAccountController::upload_configs_file),
                )
                .route(
                    "/follow-account",
                    post(DeboxAccountController::follow_account),
                )
                .route(
                    "/cross-account-group-invite",
                    post(DeboxAccountController::cross_account_group_invite),
                ),
        )
    }
}
