//! DeBox群组成员成员管理

use axum_context::Context;
use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    DeboxGroupMemberService,
    dto::debox_group_member::{
        CreateDeboxGroupMemberReq, CreateDeboxGroupMemberResp, DeleteDeboxGroupMemberReq,
        DeleteDeboxGroupMemberResp, GetDeboxGroupMemberReq, GetDeboxGroupMemberResp,
        GetDeboxGroupMembersReq, GetDeboxGroupMembersResp, SyncDeboxGroupMemberReq,
        SyncDeboxGroupMemberResp, UpdateDeboxGroupMemberReq, UpdateDeboxGroupMemberResp,
        UpdateDeboxGroupMemberStatusReq, UpdateDeboxGroupMemberStatusResp,
    },
};

/// 控制器
pub struct DeboxGroupMemberController;

impl DeboxGroupMemberController {
    /// 获取DeBox群组成员列表
    pub async fn list(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<GetDeboxGroupMembersReq>,
    ) -> Responder<GetDeboxGroupMembersResp> {
        let debox_group_service: DeboxGroupMemberService = provider.provide();
        let (results, total) = debox_group_service.list(&ctx, req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取DeBox群组成员信息
    pub async fn info(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxGroupMemberReq>,
    ) -> Responder<GetDeboxGroupMemberResp> {
        let debox_group_service: DeboxGroupMemberService = provider.provide();
        let result = debox_group_service.info(&ctx, req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加DeBox群组成员
    pub async fn create(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxGroupMemberReq>,
    ) -> Responder<CreateDeboxGroupMemberResp> {
        let debox_group_service: DeboxGroupMemberService = provider.provide();
        let _result = debox_group_service.create(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox群组成员
    pub async fn update(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxGroupMemberReq>,
    ) -> Responder<UpdateDeboxGroupMemberResp> {
        let debox_group_service: DeboxGroupMemberService = provider.provide();
        let _result = debox_group_service.update(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox群组成员状态
    pub async fn update_status(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxGroupMemberStatusReq>,
    ) -> Responder<UpdateDeboxGroupMemberStatusResp> {
        let debox_group_service: DeboxGroupMemberService = provider.provide();
        debox_group_service.update_status(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除DeBox群组成员
    pub async fn delete(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDeboxGroupMemberReq>,
    ) -> Responder<DeleteDeboxGroupMemberResp> {
        let debox_group_service: DeboxGroupMemberService = provider.provide();
        let _result = debox_group_service.delete(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

impl DeboxGroupMemberController {
    /// 同步DeBox群组成员列表
    pub async fn sync_group_members(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<SyncDeboxGroupMemberReq>,
    ) -> Responder<SyncDeboxGroupMemberResp> {
        let debox_group_service: DeboxGroupMemberService = provider.provide();
        debox_group_service.sync_group_members(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}
