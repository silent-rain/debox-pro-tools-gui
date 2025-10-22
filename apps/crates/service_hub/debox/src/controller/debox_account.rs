//! DeBox账号管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::config::{
        CreateDeboxAccountReq, CreateDeboxAccountResp, DeleteDeboxAccountReq, DeleteDeboxAccountResp, GetDeboxAccountReq,
        GetDeboxAccountResp, GetDeboxAccountTreeReq, GetDeboxAccountTreeResp, GetDeboxAccountsReq, GetDeboxAccountsResp,
        UpdateDeboxAccountReq, UpdateDeboxAccountResp, UpdateDeboxAccountStatusReq, UpdateDeboxAccountStatusResp,
    },
    service::config::DeboxAccountService,
};

/// 控制器
pub struct DeboxAccountController;

impl DeboxAccountController {
    /// 获配置列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxAccountsReq>,
    ) -> Responder<GetDeboxAccountsResp> {
        let config_service: DeboxAccountService = provider.provide();
        let (results, total) = config_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取配置树列表
    pub async fn tree(
        Extension(provider): Extension<AInjectProvider>,
        Query(_req): Query<GetDeboxAccountTreeReq>,
    ) -> Responder<GetDeboxAccountTreeResp> {
        let config_service: DeboxAccountService = provider.provide();
        let result = config_service.tree().await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 获取配置信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxAccountReq>,
    ) -> Responder<GetDeboxAccountResp> {
        let config_service: DeboxAccountService = provider.provide();
        let result = config_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加配置
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxAccountReq>,
    ) -> Responder<CreateDeboxAccountResp> {
        let config_service: DeboxAccountService = provider.provide();
        let _result = config_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新配置
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountReq>,
    ) -> Responder<UpdateDeboxAccountResp> {
        let config_service: DeboxAccountService = provider.provide();
        let _result = config_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新配置状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountStatusReq>,
    ) -> Responder<UpdateDeboxAccountStatusResp> {
        let config_service: DeboxAccountService = provider.provide();
        config_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除配置
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDeboxAccountReq>,
    ) -> Responder<DeleteDeboxAccountResp> {
        let config_service: DeboxAccountService = provider.provide();
        let _result = config_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
