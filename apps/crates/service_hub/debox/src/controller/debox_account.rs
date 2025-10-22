//! DeBox账号管理

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Query};

use inject::AInjectProvider;

use crate::{
    dto::debox_account::{
        CreateDeboxAccountReq, CreateDeboxAccountResp, DeleteDeboxAccountReq,
        DeleteDeboxAccountResp, GetDeboxAccountReq, GetDeboxAccountResp, GetDeboxAccountsReq,
        GetDeboxAccountsResp, UpdateDeboxAccountReq, UpdateDeboxAccountResp,
        UpdateDeboxAccountStatusReq, UpdateDeboxAccountStatusResp,
    },
    service::debox_account::DeboxAccountService,
};

/// 控制器
pub struct DeboxAccountController;

impl DeboxAccountController {
    /// 获DeBox账号列表
    pub async fn list(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxAccountsReq>,
    ) -> Responder<GetDeboxAccountsResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let (results, total) = debox_account_service.list(req).await?;

        let resp = Response::data_list(results, total).to_json()?;
        Ok(resp)
    }

    /// 获取DeBox账号信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Query(req): Query<GetDeboxAccountReq>,
    ) -> Responder<GetDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let result = debox_account_service.info(req).await?;

        let resp = Response::data(result).to_json()?;
        Ok(resp)
    }

    /// 添加DeBox账号
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxAccountReq>,
    ) -> Responder<CreateDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let _result = debox_account_service.create(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新DeBox账号
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountReq>,
    ) -> Responder<UpdateDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let _result = debox_account_service.update(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 更新DeBox账号状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountStatusReq>,
    ) -> Responder<UpdateDeboxAccountStatusResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        debox_account_service.update_status(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }

    /// 删除DeBox账号
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<DeleteDeboxAccountReq>,
    ) -> Responder<DeleteDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let _result = debox_account_service.delete(req).await?;

        let resp = Response::<()>::ok().to_json()?;
        Ok(resp)
    }
}
