//! DeBox群组管理

use log::error;
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};

use axum_context::Context;
use debox_pro_rs::{Config as DeBoxConfig, DaoExtApi, DeBoxClient, dto::dao_ext::MyDao};
use entity::debox::{debox_account, debox_group};
use err_code::{Error, ErrorMsg};

use crate::{
    DeboxAccountDao, DeboxGroupDao,
    dto::debox_group::{
        CreateDeboxGroupReq, DeleteDeboxGroupReq, GetDeboxGroupReq, GetDeboxGroupsReq,
        SyncDeboxGroupReq, UpdateDeboxGroupReq, UpdateDeboxGroupStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct DeboxGroupService {
    debox_group_dao: DeboxGroupDao,
    debox_account_dao: DeboxAccountDao,
}

impl DeboxGroupService {
    /// 获取列表数据
    pub async fn list(
        &self,
        ctx: &Context,
        req: GetDeboxGroupsReq,
    ) -> Result<(Vec<debox_group::Model>, u64), ErrorMsg> {
        let user_id = ctx.get_user_id();

        let (results, total) = self
            .debox_group_dao
            .list(user_id, req)
            .await
            .map_err(|err| {
                error!("查询DeBox群组列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox群组列表失败")
            })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        ctx: &Context,
        req: GetDeboxGroupReq,
    ) -> Result<debox_group::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_group_dao
            .info(req.id, user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox群组信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox群组信息失败")
            })?
            .ok_or_else(|| {
                error!("DeBox群组不存在");
                Error::DbQueryEmptyError.into_err_with_msg("DeBox群组不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        ctx: &Context,
        req: CreateDeboxGroupReq,
    ) -> Result<debox_group::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let model = debox_group::ActiveModel {
            user_id: Set(user_id),
            account_id: Set(req.model.account_id),
            name: Set(req.model.name),
            invite_code: Set(req.model.invite_code),
            pic: Set(req.model.pic),
            desc: Set(req.model.desc),
            status: Set(true),
            ..Default::default()
        };
        let result = self.debox_group_dao.create(model).await.map_err(|err| {
            error!("添加DeBox群组信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加DeBox群组信息失败")
        })?;

        Ok(result)
    }

    /// 更新DeBox群组
    pub async fn update(&self, ctx: &Context, req: UpdateDeboxGroupReq) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();
        let active_model = debox_group::ActiveModel {
            account_id: Set(req.model.account_id),
            name: Set(req.model.name),
            invite_code: Set(req.model.invite_code),
            pic: Set(req.model.pic),
            desc: Set(req.model.desc),
            status: Set(true),
            ..Default::default()
        };

        let result = self
            .debox_group_dao
            .update(req.model.id, user_id, active_model)
            .await
            .map_err(|err| {
                error!("更新DeBox群组失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox群组失败")
            })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(
        &self,
        ctx: &Context,
        req: UpdateDeboxGroupStatusReq,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        self.debox_group_dao
            .update_status(req.id, user_id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新DeBox群组状态失败, 该DeBox群组不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新DeBox群组状态失败, 该DeBox群组不存在");
                }
                error!("更新DeBox群组状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox群组状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, ctx: &Context, req: DeleteDeboxGroupReq) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_group_dao
            .delete(req.id, user_id)
            .await
            .map_err(|err| {
                error!("删除DeBox群组信息失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除DeBox群组信息失败")
            })?;

        Ok(result)
    }
}

impl DeboxGroupService {
    /// 获取DeBox客户端
    fn debox_client(&self, model: &debox_account::Model) -> Result<DeBoxClient, ErrorMsg> {
        let config = DeBoxConfig {
            app_id: model.app_id.clone(),
            api_key: model.api_key.clone(),
            app_secret: model.app_secret.clone(),
            access_token: model.access_token.clone(),
            web_token: model.web_token.clone(),
            user_id: model.debox_user_id.clone(),
            ..Default::default()
        };

        let client = DeBoxClient::new(config).map_err(|e| {
            error!("获取DeBox客户端失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("获取DeBox客户端失败")
        })?;
        Ok(client)
    }
}

impl DeboxGroupService {
    /// 创建或更新群组
    async fn create_or_update(
        &self,
        user_id: i32,
        account_id: i32,
        invite_code: String,
        data: &MyDao,
    ) -> Result<(), ErrorMsg> {
        // 查询dao
        let dao_info = self
            .debox_group_dao
            .info_by_gid(user_id, account_id, data.gid.clone())
            .await
            .map_err(|e| {
                error!("查询DeBox DAO信息失败, err: {:#?}", e);
                Error::DbQueryError.into_err_with_msg("查询DeBox DAO信息失败")
            })?;

        let mut active_model = debox_group::ActiveModel {
            gid: Set(data.gid.clone()),
            name: Set(data.name.clone()),
            invite_code: Set(invite_code),
            pic: Set(data.info.pic.clone()),
            ..Default::default()
        };

        match dao_info {
            Some(data) => {
                self.debox_group_dao
                    .update(data.id, data.user_id, active_model)
                    .await
                    .map_err(|e| {
                        error!("更新DeBox DAO信息失败, err: {e:#?}");
                        Error::DbUpdateError.into_err_with_msg("更新DeBox DAO信息失败")
                    })?;
            }
            None => {
                active_model.user_id = Set(user_id);
                active_model.account_id = Set(account_id);
                active_model.status = Set(true);
                self.debox_group_dao
                    .create(active_model)
                    .await
                    .map_err(|e| {
                        error!("添加DeBox DAO信息失败, err: {e:#?}");
                        Error::DbAddError.into_err_with_msg("添加DeBox DAO信息失败")
                    })?;
            }
        }

        Ok(())
    }

    /// 同步DeBox群组列表
    pub async fn sync_groups(&self, ctx: &Context, req: SyncDeboxGroupReq) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        for account_id in req.account_ids {
            // 获取账号信息
            let account_info = match self.debox_account_dao.info(account_id, user_id).await {
                Ok(Some(account_info)) => account_info,
                Ok(_account_info) => {
                    error!("DeBox账号不存在, account_id: {}", account_id);
                    continue;
                }
                Err(e) => {
                    error!("查询DeBox账号信息失败, err: {:#?}", e);
                    continue;
                }
            };

            // 生成debox客户端
            let client = match self.debox_client(&account_info) {
                Ok(client) => client,
                Err(e) => {
                    error!("获取DeBox客户端失败, err: {:#?}", e);
                    continue;
                }
            };

            // 获取debox dao列表
            let daos = match client.my_dao().await {
                Ok(daos) => daos,
                Err(e) => {
                    error!("获取DeBox DAO失败, err: {:#?}", e);
                    continue;
                }
            };

            let invite_code = account_info.invite_code;

            // https://m.debox.pro/group?id=bzqg8m3n&code=peqt8jxu
            for dao in daos {
                if self
                    .create_or_update(user_id, account_id, invite_code.clone(), &dao)
                    .await
                    .is_err()
                {
                    continue;
                }
            }
        }

        Ok(())
    }
}
