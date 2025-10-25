//! 登陆

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json};

use inject::AInjectProvider;

use crate::{
    LoginService,
    dto::login::{LoginReq, LoginResp},
};

/// 控制器
pub struct LoginController;

impl LoginController {
    /// 登陆
    pub async fn login(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<LoginReq>,
    ) -> Responder<LoginResp> {
        let login_service: LoginService = provider.provide();
        let result = login_service.login(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }
}
