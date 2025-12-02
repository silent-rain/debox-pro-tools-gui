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
    DeboxAccountDao, DeboxAccountFollowDao, DeboxGroupDao,
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
    debox_account_follow_dao: DeboxAccountFollowDao,
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
        active_model.web_token_status = Set(true);
        let user_info = self.get_debox_account(model).await.inspect_err(|_e| {
            active_model.web_token_status = Set(false);
        })?;
        let name = if user_info.name.is_empty() {
            user_info.address[user_info.address.len() - 8..].to_string()
        } else {
            user_info.name
        };

        active_model.name = Set(name);
        active_model.avatar = Set(Some(user_info.pic));
        active_model.invite_code = Set(user_info.invite_code);
        active_model.wallet_address = Set(user_info.address);

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
        if let Err(e) = self
            .update_account_info_and_status(&mut active_model, &account)
            .await
        {
            error!("更新账号状态和用户信息失败, err: {:#?}", e);

            self.debox_account_dao
                .update(account.id, account.user_id, active_model)
                .await
                .map_err(|err| {
                    error!("更新DeBox账号失败, err: {:#?}", err);
                    Error::DbUpdateError.into_err_with_msg("更新DeBox账号失败")
                })?;
            return Err(e);
        };

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
        let user_id = ctx.get_user_id();

        // 账号列表
        let data = GetDeboxAccountsReq {
            all: Some(true),
            status: Some(true),
            ..Default::default()
        };
        let (accounts, _) = self.list(ctx, data).await?;

        // 账号相互关注列表
        let mut account_follows = Vec::new(); // (主账号 model, 待关注账号)
        for account in accounts.iter() {
            // 获取主账号关注列表
            let account_followed_ids = self.get_follows_by_account(user_id, account.id).await?;

            // 待关注账号列表 = 账号列表 - 主账号 - 主账号关注列表
            let debox_user_ids: Vec<String> = accounts
                .iter()
                .map(|v| v.debox_user_id.clone())
                .filter(|v| *v != account.debox_user_id) // 过滤主账号
                .filter(|id| !account_followed_ids.contains(id)) // 过滤已关注的账号
                .collect();
            if debox_user_ids.is_empty() {
                error!("account_id: {} ref debox_user_ids is empty", account.id);
                continue;
            }

            info!(
                "account_id: {} ref debox_user_ids: {:?}",
                account.id,
                debox_user_ids.len()
            );
            account_follows.push((account.clone(), debox_user_ids));
        }

        // 开始批量关注
        for (account, debox_user_ids) in account_follows.into_iter() {
            let account_follow_total = debox_user_ids.len();
            let mut account_follow_failed = 0;
            for debox_user_id in debox_user_ids.iter() {
                if let Err(e) = self.debox_follow_account(&account, debox_user_id).await {
                    error!(
                        "debox_user_id: {} - {} 账号相互关注失败, err: {:#?}",
                        account.debox_user_id, debox_user_id, e
                    );
                    account_follow_failed += 1;
                }
                sleep(Duration::from_millis(100)).await;
            }
            if account_follow_failed > 0 {
                error!(
                    "account_id: {} 相互关注失败, 总数量: {:?}, 失败数量: {:?}",
                    account.id, account_follow_total, account_follow_failed
                );
            }
        }

        Ok(())
    }

    /// 获取号的关注人列表
    async fn get_follows_by_account(
        &self,
        user_id: i32,
        account_id: i32,
    ) -> Result<Vec<String>, ErrorMsg> {
        let followeds = self
            .debox_account_follow_dao
            .follows_by_account_id(user_id, account_id)
            .await
            .map_err(|err| {
                error!("查询账号的关注人列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询账号的关注人列表失败")
            })?;

        let followed_ids: Vec<String> = followeds
            .into_iter()
            .map(|followed| followed.debox_user_id)
            .collect();

        Ok(followed_ids)
    }

    /// 账号之间的群组相互拉群
    pub async fn cross_account_group_invite(&self, ctx: &Context) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        // 账号列表
        let data = GetDeboxAccountsReq {
            all: Some(true),
            status: Some(true),
            ..Default::default()
        };
        let (accounts, _) = self.list(ctx, data).await?;

        // 群组列表
        let groups = self.group_list(ctx).await?;

        // 按群组进行账号分组
        let mut group_account_ids_map = HashMap::new();
        for group in groups.iter() {
            group_account_ids_map
                .entry(group.gid.clone())
                .or_insert(Vec::new())
                .push(group.account_id);
        }

        println!("====1 group_account_ids_map: {:?}", group_account_ids_map);

        for (k, v) in group_account_ids_map.iter() {
            println!("====1  gid: {} used_account_ids: {:?}", k, v);
        }

        let mut account_groups = Vec::new(); // (主账号 model, 群ID, 待加群账号列表)
        for account in accounts.iter() {
            let groups = self.get_group_by_account(user_id, account.id).await?;

            // 遍历群组列表
            for group in groups.iter() {
                // 已经添加过该群的账号
                let used_account_ids = match group_account_ids_map.get(&group.gid) {
                    Some(v) => v.clone(),
                    None => {
                        info!(
                            "account_id: {} gid: {} has not added any users",
                            account.id, group.gid
                        );
                        continue;
                    }
                };

                // 待加群组账号列表 = 账号列表 - 主账号 - 已加群的账号
                let debox_user_ids: Vec<String> = accounts
                    .iter()
                    .filter(|v| *v.debox_user_id != account.debox_user_id) // 过滤主账号
                    .filter(|v| !used_account_ids.contains(&v.id)) // 过滤已加群的账号
                    .map(|v| v.debox_user_id.clone())
                    .collect();
                if debox_user_ids.is_empty() {
                    error!(
                        "account_id: {} gid: {} ref debox_user_ids is empty",
                        account.id, group.gid
                    );
                    continue;
                }

                info!(
                    "account_id: {} gid: {} ref debox_user_ids: {:?}",
                    account.id,
                    group.gid,
                    debox_user_ids.len()
                );
                account_groups.push((account.clone(), group.gid.clone(), debox_user_ids));
            }
        }

        // 开始批量拉群
        if account_groups.is_empty() {
            info!("account_groups is empty");
            return Ok(());
        }
        for (account, gid, debox_user_ids) in account_groups.into_iter() {
            info!(
                "account_id: {} gid: {} - {:#?} 拉群开始",
                account.id, gid, debox_user_ids
            );

            // 拉群
            if let Err(e) = self
                .group_add_member(&account, &gid, debox_user_ids.clone())
                .await
            {
                error!(
                    "account_id: {} gid: {} - {:#?} 拉群失败, err: {:#?}",
                    account.id, gid, debox_user_ids, e
                );
                continue;
            }

            // 将拉群结果保存到数据库
            if let Err(e) = self
                .add_group_to_account(ctx, &account, gid.clone(), debox_user_ids.clone())
                .await
            {
                error!(
                    "account_id: {} gid: {} - {:#?} 保存拉群结果失败, err: {:#?}",
                    account.id, gid, debox_user_ids, e
                );
            } else {
                info!(
                    "account_id: {} gid: {} - {:#?} 拉群成功",
                    account.id, gid, debox_user_ids
                );
            }

            sleep(Duration::from_millis(100)).await;
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
    /// 获取号的群组列表
    async fn get_group_by_account(
        &self,
        user_id: i32,
        account_id: i32,
    ) -> Result<Vec<debox_group::Model>, ErrorMsg> {
        let groups = self
            .debox_group_dao
            .list_by_account_id(user_id, account_id)
            .await
            .map_err(|err| {
                error!("查询账号的群组列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询账号的群组列表失败")
            })?;

        Ok(groups)
    }

    /// 将拉群结果保存到数据库， 将新增的群组添加到账号的群组列表中
    async fn add_group_to_account(
        &self,
        ctx: &Context,
        account: &debox_account::Model,
        gid: String,
        debox_user_ids: Vec<String>,
    ) -> Result<(), ErrorMsg> {
        if debox_user_ids.is_empty() {
            return Ok(());
        }

        // 账号列表
        let data = GetDeboxAccountsReq {
            all: Some(true),
            status: Some(true),
            ..Default::default()
        };
        let (accounts, _) = self.list(ctx, data).await?;

        // 群组列表
        let groups = self
            .get_group_by_account(account.user_id, account.id)
            .await?;

        let mut gid_map = HashMap::new();
        for group in groups.iter() {
            gid_map.entry(group.gid.clone()).or_insert(group.clone());
        }

        // 获取群组信息
        let group = match gid_map.get(&gid) {
            Some(v) => v,
            None => {
                error!("account_id: {} gid: {} not found", account.id, gid);
                return Ok(());
            }
        };

        // 目标账号
        let target_account_ids: Vec<i32> = accounts
            .iter()
            .filter(|v| debox_user_ids.contains(&v.debox_user_id))
            .map(|v| v.id)
            .collect();

        for target_account_id in target_account_ids {
            // 创建dao
            let active_model = debox_group::ActiveModel {
                user_id: Set(account.user_id),
                account_id: Set(target_account_id),
                gid: Set(gid.clone()),
                name: Set(group.name.clone()),
                pic: Set(group.pic.clone()),
                // invite_code: Set(invite_code.clone()),
                status: Set(true),
                ..Default::default()
            };
            self.debox_group_dao
                .create(active_model.clone())
                .await
                .map_err(|e| {
                    error!("添加DeBox DAO信息失败, data: {active_model:#?}, err: {e:#?}");
                    Error::DbAddError.into_err_with_msg("添加DeBox DAO信息失败")
                })?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_account_ids_map() {
        struct Group {
            id: i32,
            gid: String,
        }

        let groups = [
            Group {
                id: 1,
                gid: "gid1".to_string(),
            },
            Group {
                id: 2,
                gid: "gid2".to_string(),
            },
            Group {
                id: 3,
                gid: "gid1".to_string(),
            },
            Group {
                id: 3,
                gid: "gid2".to_string(),
            },
        ];

        let mut group_account_ids_map = HashMap::new();
        for group in groups.iter() {
            group_account_ids_map
                .entry(group.gid.clone())
                .or_insert(Vec::new())
                .push(group.id);
        }
        println!("{:?}", group_account_ids_map)
    }
}
