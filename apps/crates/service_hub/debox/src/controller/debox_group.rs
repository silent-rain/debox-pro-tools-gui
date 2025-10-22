//! DeBox群组管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::debox_group::{
        CreateDeboxGroupReq, CreateDeboxGroupResp, DeleteDeboxGroupReq, DeleteDeboxGroupResp,
        GetDeboxGroupReq, GetDeboxGroupResp, GetDeboxGroupsReq, GetDeboxGroupsResp,
        UpdateDeboxGroupReq, UpdateDeboxGroupResp, UpdateDeboxGroupStatusReq,
        UpdateDeboxGroupStatusResp,
    },
    service::debox_group::DeboxGroupService,
};

/// 控制器
pub struct DeboxGroupController;

impl DeboxGroupController {
    /// 获DeBox群组列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxGroupsReq>,
    ) -> Responder<GetDeboxGroupsResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let (results, total) = debox_group_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取DeBox群组信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxGroupReq>,
    ) -> Responder<GetDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let result = debox_group_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加DeBox群组
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxGroupReq>,
    ) -> Responder<CreateDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let _result = debox_group_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新DeBox群组
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxGroupReq>,
    ) -> Responder<UpdateDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let _result = debox_group_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新DeBox群组状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxGroupStatusReq>,
    ) -> Responder<UpdateDeboxGroupStatusResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        debox_group_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除DeBox群组
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDeboxGroupReq>,
    ) -> Responder<DeleteDeboxGroupResp> {
        let debox_group_service: DeboxGroupService = provider.provide();
        let _result = debox_group_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
