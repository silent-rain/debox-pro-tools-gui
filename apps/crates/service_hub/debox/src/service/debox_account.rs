//! DeBox账号管理

use log::error;
use nject::injectable;
use sea_orm::{ActiveValue::NotSet, DbErr::RecordNotUpdated, IntoActiveModel};

use entity::debox::debox_account;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::debox_account::DeboxAccountDao,
    dto::debox_account::{
        CreateDeboxAccountReq, DeleteDeboxAccountReq, GetDeboxAccountReq, GetDeboxAccountsReq,
        UpdateDeboxAccountReq, UpdateDeboxAccountStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct DeboxAccountService {
    debox_account_dao: DeboxAccountDao,
}

impl DeboxAccountService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetDeboxAccountsReq,
    ) -> Result<(Vec<debox_account::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.debox_account_dao.all().await.map_err(|err| {
                error!("查询DeBox账号列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号列表失败")
            });
        }

        let (results, total) = self.debox_account_dao.list(req).await.map_err(|err| {
            error!("查询DeBox账号列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询DeBox账号列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetDeboxAccountReq) -> Result<debox_account::Model, ErrorMsg> {
        let result = self
            .debox_account_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号信息失败")
            })?
            .ok_or_else(|| {
                error!("DeBox账号不存在");
                Error::DbQueryEmptyError.into_err_with_msg("DeBox账号不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        req: CreateDeboxAccountReq,
    ) -> Result<debox_account::Model, ErrorMsg> {
        // let model = debox_account::ActiveModel {
        //     user_id: Set(req.user_id),
        //     api_key: Set(req.api_key.clone()),
        //     app_secret: Set(req.app_secret.clone()),
        //     access_token: Set(req.access_token.clone()),
        //     web_token: Set(req.web_token.clone()),
        //     debox_user_id: Set(req.debox_user_id.clone()),
        //     wallet_address: Set(req.wallet_address.clone()),
        //     api_key_status: Set(req.api_key_status),
        //     access_token_status: Set(req.access_token_status),
        //     web_token_status: Set(req.web_token_status),
        //     desc: Set(req.desc.clone()),
        //     status: Set(true),
        //     ..Default::default()
        // };

        let mut model = req.model.into_active_model();
        model.id = NotSet;
        model.created_at = NotSet;
        model.updated_at = NotSet;

        let result = self.debox_account_dao.create(model).await.map_err(|err| {
            error!("添加DeBox账号信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加DeBox账号信息失败")
        })?;

        Ok(result)
    }

    /// 更新DeBox账号
    pub async fn update(&self, req: UpdateDeboxAccountReq) -> Result<u64, ErrorMsg> {
        // let model = debox_account::ActiveModel {
        //     id: Set(req.id),
        //     user_id: Set(req.user_id),
        //     api_key: Set(req.api_key.clone()),
        //     app_secret: Set(req.app_secret.clone()),
        //     access_token: Set(req.access_token.clone()),
        //     web_token: Set(req.web_token.clone()),
        //     debox_user_id: Set(req.debox_user_id.clone()),
        //     wallet_address: Set(req.wallet_address.clone()),
        //     api_key_status: Set(req.api_key_status),
        //     access_token_status: Set(req.access_token_status),
        //     web_token_status: Set(req.web_token_status),
        //     desc: Set(req.desc.clone()),
        //     status: Set(req.status),
        //     ..Default::default()
        // };
        let mut model = req.model.into_active_model();
        model.created_at = NotSet;
        model.updated_at = NotSet;

        let result = self.debox_account_dao.update(model).await.map_err(|err| {
            error!("更新DeBox账号失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新DeBox账号失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateDeboxAccountStatusReq) -> Result<(), ErrorMsg> {
        self.debox_account_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新DeBox账号状态失败, 该DeBox账号不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新DeBox账号状态失败, 该DeBox账号不存在");
                }
                error!("更新DeBox账号状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox账号状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteDeboxAccountReq) -> Result<u64, ErrorMsg> {
        let result = self.debox_account_dao.delete(req.id).await.map_err(|err| {
            error!("删除DeBox账号信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除DeBox账号信息失败")
        })?;

        Ok(result)
    }
}
