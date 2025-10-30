//! DeBox账号管理

use log::error;
use nject::injectable;
use sea_orm::{
    ActiveValue::{NotSet, Set},
    DbErr::RecordNotUpdated,
    IntoActiveModel,
};

use debox_pro_rs::{
    Config as DeBoxConfig, DeBoxClient, UserApi, UserExtApi,
    dto::{
        user::{IsUserFollowReq, UserInfoReq},
        user_ext::{self, UserInfo},
    },
};
use entity::debox::debox_account;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::debox_account::DeboxAccountDao,
    dto::debox_account::{
        CreateDeboxAccountReq, DeleteDeboxAccountReq, GetDeboxAccountReq, GetDeboxAccountsReq,
        UpdateAccountInfoReq, UpdateAllAccountsInfoReq, UpdateDeboxAccountReq,
        UpdateDeboxAccountStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct DeboxAccountService {
    debox_account_dao: DeboxAccountDao,
}

impl DeboxAccountService {
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

    /// 检查 ApiKey 状态
    async fn check_api_key_status(&self, model: &debox_account::Model) -> Result<(), ErrorMsg> {
        let client = self.debox_client(model)?;

        let data = IsUserFollowReq {
            wallet_address: "0xe409b19729ed02ca6a2b05f4d2cdae86b6a0ddbd".to_string(),
            follow_address: "0x0f4c6380a3864ced10ee1064f6a0d21233880c5d".to_string(),
        };
        let _resp = client.is_user_follow(data).await.map_err(|e| {
            error!("Api Key 状态检查失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("Api Key 状态检查失败")
        })?;

        Ok(())
    }

    /// 检查 Access Token 状态
    async fn check_access_token_status(
        &self,
        model: &debox_account::Model,
    ) -> Result<(), ErrorMsg> {
        let client = self.debox_client(model)?;

        let data = UserInfoReq {
            user_id: "2y9u8fkw".to_string(),
        };
        let _resp = UserApi::user_info(&client, data).await.map_err(|e| {
            error!("Access Token 状态检查失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("Access Token 状态检查失败")
        })?;

        Ok(())
    }

    /// 检查 Web Token 状态
    async fn _check_web_token_status(&self, model: &debox_account::Model) -> Result<(), ErrorMsg> {
        let _resp = self.get_debox_account(model).await.map_err(|e| {
            error!(", err: {:#?}", e);
            e
        })?;

        Ok(())
    }

    /// 获取 debox 账号信息
    async fn get_debox_account(&self, model: &debox_account::Model) -> Result<UserInfo, ErrorMsg> {
        let client = self.debox_client(model)?;

        let data = user_ext::UserInfoReq {
            user_id: "621674807658095".to_string(),
            iversion: 1,
            use_menu: 1,
        };
        let resp = UserExtApi::user_info(&client, data).await.map_err(|e| {
            error!("获取 DeBox 账号信息失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("获取 DeBox 账号信息失败")
        })?;

        Ok(resp)
    }
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
        // 创建用户
        let mut model = req.model.clone().into_active_model();
        model.id = NotSet;
        model.created_at = NotSet;
        model.updated_at = NotSet;

        // 账号检测
        if self.check_api_key_status(&req.model).await.is_ok() {
            model.api_key_status = Set(true)
        }
        if self.check_access_token_status(&req.model).await.is_ok() {
            model.access_token_status = Set(true)
        }

        if let Ok(user_info) = self.get_debox_account(&req.model).await {
            model.web_token_status = Set(true);
            model.name = Set(user_info.name);
            model.avatar = Set(Some(user_info.pic));
            model.wallet_address = Set(user_info.address);
        }

        let result = self.debox_account_dao.create(model).await.map_err(|err| {
            error!("添加DeBox账号信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加DeBox账号信息失败")
        })?;

        Ok(result)
    }

    /// 更新DeBox账号
    pub async fn update(&self, req: UpdateDeboxAccountReq) -> Result<u64, ErrorMsg> {
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

impl DeboxAccountService {
    /// 更新所有账户信息
    pub async fn update_all_accounts_info(
        &self,
        req: UpdateAllAccountsInfoReq,
    ) -> Result<(), ErrorMsg> {
        let (mut accounts, _) = self
            .debox_account_dao
            .accounts_by_user_id(req.user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号列表失败")
            })?;

        let total = accounts.len();
        let mut failed_count = 0;
        for account in accounts.iter_mut() {
            if self.check_api_key_status(account).await.is_ok() {
                account.api_key_status = true;
            }
            if self.check_access_token_status(account).await.is_ok() {
                account.access_token_status = true;
            }

            if let Ok(user_info) = self.get_debox_account(account).await {
                account.web_token_status = true;
                account.name = user_info.name;
                account.avatar = Some(user_info.pic);
                account.wallet_address = user_info.address;
            }

            if let Err(e) = self
                .update(UpdateDeboxAccountReq {
                    model: account.clone(),
                })
                .await
            {
                error!("更新DeBox账号失败, err: {e:#?}");
                failed_count += 1;
            }
        }

        if failed_count > 0 {
            error!("更新DeBox账号失败, 总数量: {total:?}, 失败数量: {failed_count:?}");
        }

        Ok(())
    }

    /// 更新账户信息
    pub async fn update_account_info(&self, req: UpdateAccountInfoReq) -> Result<(), ErrorMsg> {
        let mut account = self.info(GetDeboxAccountReq { id: req.id }).await?;

        if self.check_api_key_status(&account).await.is_ok() {
            account.api_key_status = true;
        }
        if self.check_access_token_status(&account).await.is_ok() {
            account.access_token_status = true;
        }

        if let Ok(user_info) = self.get_debox_account(&account).await {
            account.web_token_status = true;
            account.name = user_info.name;
            account.avatar = Some(user_info.pic);
            account.wallet_address = user_info.address;
        }

        self.update(UpdateDeboxAccountReq {
            model: account.clone(),
        })
        .await
        .map_err(|err| {
            error!("更新DeBox账号失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新DeBox账号失败")
        })?;

        Ok(())
    }
}
