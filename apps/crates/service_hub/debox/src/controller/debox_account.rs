//! DeBox账号管理

use axum::{body::Body, http::header};
use axum_context::Context;
use axum_typed_multipart::TypedMultipart;
use bytes::Bytes;
use serde_json::json;

use axum_response::{Responder, Response, ResponseErr};
use axum_validator::{Extension, Json, Path, Query};
use err_code::Error;
use inject::AInjectProvider;

use crate::{
    dto::debox_account::{
        CreateDeboxAccountReq, CreateDeboxAccountResp, DeleteDeboxAccountReq,
        DeleteDeboxAccountResp, DownloadConfigFileReq, GetDeboxAccountReq, GetDeboxAccountResp,
        GetDeboxAccountsReq, GetDeboxAccountsResp, UpdateAccountInfoReq, UpdateAccountInfoResp,
        UpdateAllAccountsInfoReq, UpdateAllAccountsInfoResp, UpdateDeboxAccountReq,
        UpdateDeboxAccountResp, UpdateDeboxAccountStatusReq, UpdateDeboxAccountStatusResp,
        UploadConfigFileReq, UploadConfigFileResp,
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

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取DeBox账号信息
    pub async fn info(
        Extension(provider): Extension<AInjectProvider>,
        Path(req): Path<GetDeboxAccountReq>,
    ) -> Responder<GetDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let result = debox_account_service.info(req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加DeBox账号
    pub async fn create(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxAccountReq>,
    ) -> Responder<CreateDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let _result = debox_account_service.create(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox账号
    pub async fn update(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountReq>,
    ) -> Responder<UpdateDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let _result = debox_account_service.update(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox账号状态
    pub async fn update_status(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountStatusReq>,
    ) -> Responder<UpdateDeboxAccountStatusResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        debox_account_service.update_status(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除DeBox账号
    pub async fn delete(
        Extension(provider): Extension<AInjectProvider>,
        Path(req): Path<DeleteDeboxAccountReq>,
    ) -> Responder<DeleteDeboxAccountResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let _result = debox_account_service.delete(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

impl DeboxAccountController {
    /// 更新所有账户信息
    pub async fn update_all_accounts_info(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateAllAccountsInfoReq>,
    ) -> Responder<UpdateAllAccountsInfoResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        debox_account_service.update_all_accounts_info(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新账户信息
    pub async fn update_account_info(
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateAccountInfoReq>,
    ) -> Responder<UpdateAccountInfoResp> {
        let debox_account_service: DeboxAccountService = provider.provide();
        debox_account_service.update_account_info(req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 下载配置文件
    /// TODO 可能需要放开权限管控或前端进行特性处理, 放开code检验
    pub async fn download_config_file(
        Extension(provider): Extension<AInjectProvider>,
        Path(req): Path<DownloadConfigFileReq>,
    ) -> Result<axum::response::Response<Body>, ResponseErr> {
        let debox_account_service: DeboxAccountService = provider.provide();
        let result = debox_account_service
            .info(GetDeboxAccountReq { id: req.id })
            .await?;

        let filename = if result.name.is_empty() {
            "config".to_string()
        } else {
            result.name
        };
        let extension = "json";
        let file_bytes = Bytes::from(serde_json::to_vec(&json!({
            "app_id": result.app_id,
            "api_key": result.api_key,
            "app_secret": result.app_secret,
            "access_token": result.access_token,
            "web_token": result.web_token,
            "debox_user_id": result.debox_user_id,
        }))?);

        // 文件名称
        let content_disposition =
            format!("attachment; filename=\"{:?}.{:?}\"", filename, extension);

        let resp = axum::response::Response::builder()
            .header("Content-Type", "application/json")
            .header(header::CONTENT_DISPOSITION, content_disposition)
            .body(Body::from(file_bytes))
            .map_err(|err| Error::InternalServer(err.to_string()).into_err())?;
        Ok(resp)
    }

    /// 上传配置文件
    pub async fn upload_config_file(
        Extension(provider): Extension<AInjectProvider>,
        ctx: Context,
        TypedMultipart(req): TypedMultipart<UploadConfigFileReq>,
    ) -> Responder<UploadConfigFileResp> {
        let user_id = ctx.get_user_id();
        let debox_account_service: DeboxAccountService = provider.provide();
        let _result = debox_account_service
            .upload_config_file(req, user_id)
            .await?;

        let resp = Response::ok();
        Ok(resp)
    }
}
