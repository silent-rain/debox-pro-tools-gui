//! DeBox账号好友管理
use axum_context::Context;

use axum_response::{Responder, Response};
use axum_validator::{Extension, Json, Path};
use inject::AInjectProvider;

use crate::{
    DeboxAccountFriendService,
    dto::debox_account_friend::{
        CreateDeboxAccountFriendReq, CreateDeboxAccountFriendResp, DeleteDeboxAccountFriendReq,
        DeleteDeboxAccountFriendResp, GetDeboxAccountFriendReq, GetDeboxAccountFriendResp,
        GetDeboxAccountFriendsReq, GetDeboxAccountFriendsResp, SyncDeboxAccountFriendsReq,
        SyncDeboxAccountFriendsResp, UpdateDeboxAccountFriendReq, UpdateDeboxAccountFriendResp,
        UpdateDeboxAccountFriendStatusReq, UpdateDeboxAccountFriendStatusResp,
    },
};

/// 控制器
pub struct DeboxAccountFriendController;

impl DeboxAccountFriendController {
    /// 获DeBox账号好友列表
    pub async fn list(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<GetDeboxAccountFriendsReq>,
    ) -> Responder<GetDeboxAccountFriendsResp> {
        let debox_account_friend_service: DeboxAccountFriendService = provider.provide();
        let (results, total) = debox_account_friend_service.list(&ctx, req).await?;

        let resp = Response::data((results, total).into());
        Ok(resp)
    }

    /// 获取DeBox账号好友信息
    pub async fn info(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Path(req): Path<GetDeboxAccountFriendReq>,
    ) -> Responder<GetDeboxAccountFriendResp> {
        let debox_account_friend_service: DeboxAccountFriendService = provider.provide();
        let result = debox_account_friend_service.info(&ctx, req).await?;

        let resp = Response::data(result.into());
        Ok(resp)
    }

    /// 添加DeBox账号好友
    pub async fn create(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<CreateDeboxAccountFriendReq>,
    ) -> Responder<CreateDeboxAccountFriendResp> {
        let debox_account_friend_service: DeboxAccountFriendService = provider.provide();
        let _result = debox_account_friend_service.create(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox账号好友
    pub async fn update(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountFriendReq>,
    ) -> Responder<UpdateDeboxAccountFriendResp> {
        let debox_account_friend_service: DeboxAccountFriendService = provider.provide();
        let _result = debox_account_friend_service.update(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 更新DeBox账号好友状态
    pub async fn update_status(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<UpdateDeboxAccountFriendStatusReq>,
    ) -> Responder<UpdateDeboxAccountFriendStatusResp> {
        let debox_account_friend_service: DeboxAccountFriendService = provider.provide();
        debox_account_friend_service
            .update_status(&ctx, req)
            .await?;

        let resp = Response::ok();
        Ok(resp)
    }

    /// 删除DeBox账号好友
    pub async fn delete(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Path(req): Path<DeleteDeboxAccountFriendReq>,
    ) -> Responder<DeleteDeboxAccountFriendResp> {
        let debox_account_friend_service: DeboxAccountFriendService = provider.provide();
        let _result = debox_account_friend_service.delete(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}

impl DeboxAccountFriendController {
    /// 同步好友列表
    pub async fn sync_friends(
        ctx: Context,
        Extension(provider): Extension<AInjectProvider>,
        Json(req): Json<SyncDeboxAccountFriendsReq>,
    ) -> Responder<SyncDeboxAccountFriendsResp> {
        let debox_account_friend_service: DeboxAccountFriendService = provider.provide();
        debox_account_friend_service.sync_friends(&ctx, req).await?;

        let resp = Response::ok();
        Ok(resp)
    }
}
