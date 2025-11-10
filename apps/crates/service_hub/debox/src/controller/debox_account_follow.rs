//! DeBox账号关注人管理
use axum_context::Context;

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Path};
use inject::AInjectProvider;

use crate::{
    DeboxAccountFollowService,
    dto::debox_account_follow::{
        CreateDeboxAccountFollowReq, CreateDeboxAccountFollowResp, DeleteDeboxAccountFollowReq,
        DeleteDeboxAccountFollowResp, GetDeboxAccountFollowReq, GetDeboxAccountFollowResp,
        GetDeboxAccountFollowsReq, GetDeboxAccountFollowsResp, UpdateDeboxAccountFollowReq,
        UpdateDeboxAccountFollowResp, UpdateDeboxAccountFollowStatusReq,
        UpdateDeboxAccountFollowStatusResp,
    },
};

/// 控制器
pub struct DeboxAccountFollowController;

impl DeboxAccountFollowController {
    /// 获DeBox账号列表
    pub async fn list(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<GetDeboxAccountFollowsReq>,
    ) -> Responder<GetDeboxAccountFollowsResp> {
        let debox_account_service: DeboxAccountFollowService = provider.provide();
        let (results, total) = debox_account_service.list(&ctx, req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取DeBox账号信息
    pub async fn info(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Path(req): Path<GetDeboxAccountFollowReq>,
    ) -> Responder<GetDeboxAccountFollowResp> {
        let debox_account_service: DeboxAccountFollowService = provider.provide();
        let result = debox_account_service.info(&ctx, req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加DeBox账号
    pub async fn create(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxAccountFollowReq>,
    ) -> Responder<CreateDeboxAccountFollowResp> {
        let debox_account_service: DeboxAccountFollowService = provider.provide();
        let _result = debox_account_service.create(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox账号
    pub async fn update(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountFollowReq>,
    ) -> Responder<UpdateDeboxAccountFollowResp> {
        let debox_account_service: DeboxAccountFollowService = provider.provide();
        let _result = debox_account_service.update(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox账号状态
    pub async fn update_status(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountFollowStatusReq>,
    ) -> Responder<UpdateDeboxAccountFollowStatusResp> {
        let debox_account_service: DeboxAccountFollowService = provider.provide();
        debox_account_service.update_status(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除DeBox账号
    pub async fn delete(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Path(req): Path<DeleteDeboxAccountFollowReq>,
    ) -> Responder<DeleteDeboxAccountFollowResp> {
        let debox_account_service: DeboxAccountFollowService = provider.provide();
        let _result = debox_account_service.delete(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}
