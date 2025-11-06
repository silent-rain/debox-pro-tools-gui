//! DeBox群组管理

use axum_context::Context;
use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::debox_group::{
        CreateDeboxGroupReq, CreateDeboxGroupResp, DeleteDeboxGroupReq, DeleteDeboxGroupResp,
        GetDeboxGroupReq, GetDeboxGroupResp, GetDeboxGroupsReq, GetDeboxGroupsResp,
        SyncDeboxGroupReq, SyncDeboxGroupResp, UpdateDeboxGroupReq, UpdateDeboxGroupResp,
        UpdateDeboxGroupStatusReq, UpdateDeboxGroupStatusResp,
    },
    service::debox_group::DeboxGroupService,
};

/// 控制器
pub struct DeboxGroupController;

impl DeboxGroupController {
    /// 获DeBox群组列表
    pub async fn list(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxGroupsReq>,
    ) -> Responder<GetDeboxGroupsResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let (results, total) = debox_group_service.list(&ctx, req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取DeBox群组信息
    pub async fn info(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxGroupReq>,
    ) -> Responder<GetDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let result = debox_group_service.info(&ctx, req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加DeBox群组
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxGroupReq>,
    ) -> Responder<CreateDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let _result = debox_group_service.create(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox群组
    pub async fn update(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxGroupReq>,
    ) -> Responder<UpdateDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let _result = debox_group_service.update(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox群组状态
    pub async fn update_status(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxGroupStatusReq>,
    ) -> Responder<UpdateDeboxGroupStatusResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        debox_group_service.update_status(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除DeBox群组
    pub async fn delete(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDeboxGroupReq>,
    ) -> Responder<DeleteDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let _result = debox_group_service.delete(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

impl DeboxGroupController {
    /// 同步DeBox群组列表
    pub async fn sync_groups(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<SyncDeboxGroupReq>,
    ) -> Responder<SyncDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        debox_group_service.sync_groups(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}
