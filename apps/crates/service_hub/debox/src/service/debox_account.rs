//! DeBox账号管理
use std::{collections::HashMap, io::Read, time::Duration};

use log::{error, info};
use nject::injectable;
use sea_orm::{ActiveValue::Set, DbErr::RecordNotUpdated};

use axum_context::Context;
use debox_pro_rs::{
    Config as DeBoxConfig, DaoExtApi, DeBoxClient, UserApi, UserExtApi,
    dto::{
        dao_ext::MemberAddReq,
        user::{IsUserFollowReq, UserInfoReq},
        user_ext::{self, FollowNewReq, UserInfo},
    },
};
use entity::debox::{debox_account, debox_group};
use err_code::{Error, ErrorMsg};
use tokio::time::sleep;
use utils::json::struct_to_struct;

use crate::{
    DeboxAccountDao, DeboxGroupDao,
    dto::{
        debox_account::{
            CreateDeboxAccountReq, DeleteDeboxAccountReq, GetDeboxAccountReq, GetDeboxAccountsReq,
            UpdateAccountInfoReq, UpdateAllAccountsInfoReq, UpdateDeboxAccountReq,
            UpdateDeboxAccountStatusReq, UploadConfigFileReq,
        },
        debox_group::GetDeboxGroupsReq,
    },
};

/// 服务层
#[injectable]
pub struct DeboxAccountService {
    debox_account_dao: DeboxAccountDao,
    debox_group_dao: DeboxGroupDao,
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

        DeBoxClient::new(config).map_err(|e| {
            error!("获取DeBox客户端失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("获取DeBox客户端失败")
        })
    }

    /// 检查 ApiKey 状态
    async fn check_api_key_status(&self, model: &debox_account::Model) -> Result<(), ErrorMsg> {
        let client = self.debox_client(model)?;

        // 使用测试钱包地址进行状态检查
        let data = IsUserFollowReq {
            wallet_address: "0xe409b19729ed02ca6a2b05f4d2cdae86b6a0ddbd".to_string(),
            follow_address: "0x0f4c6380a3864ced10ee1064f6a0d21233880c5d".to_string(),
        };

        client.is_user_follow(data).await.map_err(|e| {
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

        // 使用测试用户ID进行状态检查
        let data = UserInfoReq {
            user_id: "2y9u8fkw".to_string(),
        };

        UserApi::user_info(&client, data).await.map_err(|e| {
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
            user_id: model.debox_user_id.clone(),
            iversion: 1,
            use_menu: 1,
        };

        UserExtApi::user_info(&client, data).await.map_err(|e| {
            error!("获取 DeBox 账号信息失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("获取 DeBox 账号信息失败")
        })
    }

    /// 更新账号用户信息和状态
    async fn update_account_info_and_status(
        &self,
        active_model: &mut debox_account::ActiveModel,
        model: &debox_account::Model,
    ) -> Result<(), ErrorMsg> {
        // 检查 API Key 状态
        if self.check_api_key_status(model).await.is_ok() {
            active_model.api_key_status = Set(true);
        }

        // 检查 Access Token 状态
        if self.check_access_token_status(model).await.is_ok() {
            active_model.access_token_status = Set(true);
        }

        // 获取并更新用户信息
        let user_info = self.get_debox_account(model).await?;
        let name = if user_info.name.is_empty() {
            user_info.address[user_info.address.len() - 8..].to_string()
        } else {
            user_info.name
        };

        active_model.name = Set(name);
        active_model.avatar = Set(Some(user_info.pic));
        active_model.invite_code = Set(user_info.invite_code);
        active_model.wallet_address = Set(user_info.address);
        active_model.web_token_status = Set(true);

        Ok(())
    }

    /// 关注账号
    async fn debox_follow_account(
        &self,
        model: &debox_account::Model,
        follow_id: &str,
    ) -> Result<(), ErrorMsg> {
        let client = self.debox_client(model)?;

        let data = FollowNewReq {
            follow_id: follow_id.to_string(),
            status: 1,
        };
        client.follow_new(data).await.map_err(|e| {
            error!("关注账号:{follow_id:?} 失败, err: {e:#?}");
            Error::DeboxProRs(e).into_err_with_msg("关注账号失败")
        })?;

        Ok(())
    }

    /// 群组添加成员
    async fn group_add_member(
        &self,
        model: &debox_account::Model,
        gid: &str,
        debox_user_ids: Vec<String>,
    ) -> Result<(), ErrorMsg> {
        let client = self.debox_client(model)?;

        let data = MemberAddReq {
            gid: gid.to_string(),
            add_user_id: debox_user_ids.clone(),
        };
        client.member_add(data).await.map_err(|e| {
            error!("群组:{gid:?} 添加成员:{debox_user_ids:#?} 失败, err: {e:#?}");
            Error::DeboxProRs(e).into_err_with_msg("群组添加成员失败")
        })?;

        Ok(())
    }
}

impl DeboxAccountService {
    /// 获取列表数据
    pub async fn list(
        &self,
        ctx: &Context,
        req: GetDeboxAccountsReq,
    ) -> Result<(Vec<debox_account::Model>, u64), ErrorMsg> {
        let user_id = ctx.get_user_id();

        let (results, total) = self
            .debox_account_dao
            .list(user_id, req)
            .await
            .map_err(|err| {
                error!("查询DeBox账号列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号列表失败")
            })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        ctx: &Context,
        req: GetDeboxAccountReq,
    ) -> Result<debox_account::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_account_dao
            .info(req.id, user_id)
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

    // 是否存在 DeBox 账号
    async fn exist_debox_user_id(
        &self,
        user_id: i32,
        debox_user_id: String,
    ) -> Result<bool, ErrorMsg> {
        let result = self
            .debox_account_dao
            .account_by_debox_user_id(user_id, debox_user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号信息失败")
            })?
            .is_some();

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        ctx: &Context,
        req: CreateDeboxAccountReq,
    ) -> Result<debox_account::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        // 是否存在 DeBox 账号
        if self
            .exist_debox_user_id(user_id, req.debox_user_id.clone())
            .await?
        {
            error!(
                "DeBox账号已存在, user_id: {:#?}, debox_user_id: {:#?}",
                user_id, req.debox_user_id
            );
            return Err(Error::DbDataExistError.into_err_with_msg("DeBox账号已存在"));
        }

        // 类型转换
        let model: debox_account::Model = struct_to_struct(&req).map_err(|e| {
            error!("DeBox账号信息转换失败, err: {:#?}", e);
            Error::UtilsError(e).into_err_with_msg("DeBox账号信息转换失败")
        })?;

        // 创建基础模型
        let mut active_model = debox_account::ActiveModel {
            user_id: Set(user_id),
            app_id: Set(req.app_id.clone()),
            api_key: Set(req.api_key.clone()),
            app_secret: Set(req.app_secret.clone()),
            access_token: Set(req.access_token.clone()),
            web_token: Set(req.web_token.clone()),
            debox_user_id: Set(req.debox_user_id.clone()),
            desc: Set(req.desc.clone()),
            status: Set(req.status),
            ..Default::default()
        };

        // 更新账号状态和用户信息
        self.update_account_info_and_status(&mut active_model, &model)
            .await?;

        let result = self
            .debox_account_dao
            .create(active_model)
            .await
            .map_err(|err| {
                error!("添加DeBox账号信息失败, err: {:#?}", err);
                Error::DbAddError.into_err_with_msg("添加DeBox账号信息失败")
            })?;

        Ok(result)
    }

    /// 更新DeBox账号
    pub async fn update(&self, ctx: &Context, req: UpdateDeboxAccountReq) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();
        let id = req.id;

        // 类型转换
        let model: debox_account::Model = struct_to_struct(&req).map_err(|e| {
            error!("DeBox账号信息转换失败, err: {:#?}", e);
            Error::UtilsError(e).into_err_with_msg("DeBox账号信息转换失败")
        })?;

        let mut active_model = debox_account::ActiveModel {
            app_id: Set(req.app_id.clone()),
            api_key: Set(req.api_key.clone()),
            app_secret: Set(req.app_secret.clone()),
            access_token: Set(req.access_token.clone()),
            web_token: Set(req.web_token.clone()),
            debox_user_id: Set(req.debox_user_id.clone()),
            desc: Set(req.desc.clone()),
            status: Set(req.status),
            ..Default::default()
        };

        // 更新账号状态和用户信息
        self.update_account_info_and_status(&mut active_model, &model)
            .await?;

        self.debox_account_dao
            .update(id, user_id, active_model)
            .await
            .map_err(|err| {
                error!("更新DeBox账号失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox账号失败")
            })
    }

    /// 更新数据状态
    pub async fn update_status(
        &self,
        ctx: &Context,
        req: UpdateDeboxAccountStatusReq,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        self.debox_account_dao
            .update_status(req.id, user_id, req.status)
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
    pub async fn delete(&self, ctx: &Context, req: DeleteDeboxAccountReq) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_account_dao
            .delete(req.id, user_id)
            .await
            .map_err(|err| {
                error!("删除DeBox账号信息失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除DeBox账号信息失败")
            })?;

        Ok(result)
    }
}

impl DeboxAccountService {
    /// 更新账户信息
    pub async fn update_account_info(
        &self,
        ctx: &Context,
        req: UpdateAccountInfoReq,
    ) -> Result<(), ErrorMsg> {
        // 查询账号信息
        let account = self.info(ctx, GetDeboxAccountReq { id: req.id }).await?;

        let mut active_model = debox_account::ActiveModel {
            ..Default::default()
        };

        // 更新账号状态和用户信息
        self.update_account_info_and_status(&mut active_model, &account)
            .await?;

        self.debox_account_dao
            .update(account.id, account.user_id, active_model)
            .await
            .map_err(|err| {
                error!("更新DeBox账号失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox账号失败")
            })?;

        Ok(())
    }

    /// 更新所有账户信息
    pub async fn update_all_accounts_info(
        &self,
        ctx: &Context,
        req: UpdateAllAccountsInfoReq,
    ) -> Result<(), ErrorMsg> {
        // 查询DeBox账号列表
        let (mut accounts, _) = self
            .debox_account_dao
            .accounts_by_user_id(req.user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号列表失败")
            })?;

        if accounts.is_empty() {
            error!("DeBox账号列表为空");
            return Ok(());
        }

        // 更新所有账号信息
        let total = accounts.len();
        let mut failed_count = 0;
        for account in accounts.iter_mut() {
            if let Err(e) = self
                .update_account_info(ctx, UpdateAccountInfoReq { id: account.id })
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

    /// 上传配置文件
    ///
    /// 上传 json 文件
    /// ```json
    /// {"app_id":"","api_key":"","app_secret":"","access_token":"","web_token":"","debox_user_id":""}
    /// ```
    pub async fn upload_config_file(
        &self,
        ctx: &Context,
        mut req: UploadConfigFileReq,
    ) -> Result<debox_account::Model, ErrorMsg> {
        // 读取json文件数据
        let mut buffer = vec![];
        req.file
            .contents
            .read_to_end(&mut buffer)
            .map_err(|err| Error::UploadFileError(err.to_string()).into_err())?;

        let file_content = String::from_utf8(buffer.clone()).map_err(|e| {
            error!("文件内容转换失败, err: {:#?}", e);
            Error::FromUtf8(e).into_err_with_msg("文件内容转换失败")
        })?;

        // 添加
        let data: CreateDeboxAccountReq = serde_json::from_str(&file_content).map_err(|e| {
            error!("文件内容转换失败, err: {:#?}", e);
            Error::ConvertType(e.to_string()).into_err_with_msg("文件内容转换失败")
        })?;
        let result = self.create(ctx, data).await?;

        Ok(result)
    }

    /// 账号相互关注
    pub async fn follow_account(&self, ctx: &Context) -> Result<(), ErrorMsg> {
        // 账号列表
        let data = GetDeboxAccountsReq {
            all: Some(true),
            status: Some(true),
            ..Default::default()
        };
        let (accounts, _) = self.list(ctx, data).await?;

        // 正向账号匹配
        let mut pairs = Vec::new();
        for (i, id1) in accounts.iter().enumerate() {
            for id2 in accounts[i + 1..].iter().clone() {
                pairs.push((id1.clone(), id2.clone()));
            }
        }
        // 反向账号匹配
        let reversed_accounts: Vec<_> = accounts.iter().rev().collect(); // 将账号列表反转
        for (i, id1) in reversed_accounts.iter().enumerate() {
            for id2 in reversed_accounts[i + 1..].iter() {
                pairs.push(((*id1).clone(), (*id2).clone()));
            }
        }

        // 开始批量关注
        let total = pairs.len();
        let mut failed_count = 0;
        for (id1, id2) in pairs.iter() {
            if let Err(e) = self.debox_follow_account(id1, &id2.debox_user_id).await {
                error!(
                    "debox_user_id: {} - {} 账号相互关注失败, err: {:#?}",
                    id1.debox_user_id, id2.debox_user_id, e
                );
                failed_count += 1;
            }
            sleep(Duration::from_millis(100)).await;
        }

        if failed_count > 0 {
            error!("账号相互关注失败, 总数量: {total:?}, 失败数量: {failed_count:?}");
        }

        Ok(())
    }

    /// 账号之间的群组相互拉群
    pub async fn cross_account_group_invite(&self, ctx: &Context) -> Result<(), ErrorMsg> {
        // 账号列表
        let data = GetDeboxAccountsReq {
            all: Some(true),
            status: Some(true),
            ..Default::default()
        };
        let (accounts, _) = self.list(ctx, data).await?;

        // 群组列表
        let groups = self.group_list(ctx).await?;
        // 按账号进行群组分组
        let mut account_groups_map = HashMap::new();
        for group in groups {
            account_groups_map
                .entry(group.account_id)
                .or_insert(Vec::new())
                .push(group.clone());
        }

        // 账号群组匹配
        let mut pairs = Vec::new();
        for (i, id1) in accounts.iter().enumerate() {
            let mut debox_user_ids = Vec::new();
            for id2 in accounts[i + 1..].iter().clone() {
                debox_user_ids.push(id2.debox_user_id.clone());
            }
            if debox_user_ids.is_empty() {
                error!("account_id: {} ref debox_user_ids is empty", id1.id);
                continue;
            }
            pairs.push((id1, debox_user_ids));
        }

        error!("pairs: {:#?}", pairs);

        // 开始批量加群
        for (id1, debox_user_ids) in pairs.into_iter() {
            // 获取群组列表
            let groups = match account_groups_map.get(&id1.id) {
                Some(groups) => groups.clone(),
                None => {
                    error!("账号 {}-{} 没有群组", id1.id, id1.name);
                    continue;
                }
            };

            // 开始批量加群
            for group in groups {
                if let Err(e) = self
                    .group_add_member(id1, &group.gid, debox_user_ids.clone())
                    .await
                {
                    error!(
                        "debox_user_id: {} gid: {} - {:#?} 拉群失败, err: {:#?}",
                        id1.debox_user_id, group.gid, debox_user_ids, e
                    );
                }
                info!(
                    "debox_user_id: {} gid: {} - {:#?} 拉群成功",
                    id1.debox_user_id, group.gid, debox_user_ids
                );
            }
        }

        Ok(())
    }

    /// 获取群组列表数据
    async fn group_list(&self, ctx: &Context) -> Result<Vec<debox_group::Model>, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let data = GetDeboxGroupsReq {
            all: Some(true),
            status: Some(true),
            ..Default::default()
        };

        let (results, _total) = self
            .debox_group_dao
            .list(user_id, data)
            .await
            .map_err(|err| {
                error!("查询DeBox群组列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox群组列表失败")
            })?;

        Ok(results)
    }
}
