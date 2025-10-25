//! 登出

use axum_context::Context;
use axum_validator::{Extension, Json};

use axum_response::{Responder, Response};
use inject::AInjectProvider;

use crate::{
    dto::logout::{LogoutReq, LogoutResp},
    service::logout::Logoutervice,
};

/// 控制器
pub struct LogoutController;

impl LogoutController {
    /// 登出
    pub async fn logout(
        Extension(provider): Extension<AInjectProvider>,
        ctx: Context,
        Json(_req): Json<LogoutReq>,
    ) -> Responder<LogoutResp> {
        let login_service: Logoutervice = provider.provide();
        login_service.logout(ctx).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
