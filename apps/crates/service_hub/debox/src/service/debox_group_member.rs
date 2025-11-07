//! DeBox群组成员管理

use std::{collections::HashMap, sync::Arc};

use futures::future::join_all;
use log::{error, info};
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};
use tokio::sync::Semaphore;

use axum_context::Context;
use debox_pro_rs::{
    Config as DeBoxConfig, DaoExtApi, DeBoxClient,
    dto::dao_ext::{DaoMember, DaoMemberReq},
};
use entity::debox::{debox_account, debox_group, debox_group_member};
use err_code::{Error, ErrorMsg};

use crate::{
    DeboxAccountDao, DeboxGroupDao, DeboxGroupMemberDao,
    dto::debox_group_member::{
        CreateDeboxGroupMemberReq, DeleteDeboxGroupMemberReq, GetDeboxGroupMemberReq,
        GetDeboxGroupMembersReq, SyncDeboxGroupMemberReq, UpdateDeboxGroupMemberReq,
        UpdateDeboxGroupMemberStatusReq,
    },
};

const CONCURRENCY_LIMIT: usize = 2; // 控制并发数
const SYNC_PAGE_LIMIT: usize = 10; // 同步页面数

/// 服务层
#[injectable]
#[derive(Clone)]
pub struct DeboxGroupMemberService {
    debox_group_member_dao: DeboxGroupMemberDao,
    debox_group_dao: DeboxGroupDao,
    debox_account_dao: DeboxAccountDao,
}

impl DeboxGroupMemberService {
    /// 获取列表数据
    pub async fn list(
        &self,
        ctx: &Context,
        req: GetDeboxGroupMembersReq,
    ) -> Result<(Vec<debox_group_member::Model>, u64), ErrorMsg> {
        let user_id = ctx.get_user_id();

        let (results, total) = self
            .debox_group_member_dao
            .list(user_id, req)
            .await
            .map_err(|err| {
                error!("查询DeBox群组成员列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox群组成员列表失败")
            })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        ctx: &Context,
        req: GetDeboxGroupMemberReq,
    ) -> Result<debox_group_member::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_group_member_dao
            .info(req.id, user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox群组成员信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox群组成员信息失败")
            })?
            .ok_or_else(|| {
                error!("DeBox群组成员不存在");
                Error::DbQueryEmptyError.into_err_with_msg("DeBox群组成员不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        ctx: &Context,
        req: CreateDeboxGroupMemberReq,
    ) -> Result<debox_group_member::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let model = debox_group_member::ActiveModel {
            user_id: Set(user_id),
            account_id: Set(req.account_id),
            group_id: Set(req.group_id),
            group_gid: Set(req.group_gid),
            debox_user_id: Set(req.debox_user_id),
            address: Set(req.address),
            name: Set(req.name),
            pic: Set(req.pic),
            ..Default::default()
        };
        let result = self
            .debox_group_member_dao
            .create(model)
            .await
            .map_err(|err| {
                error!("添加DeBox群组成员信息失败, err: {:#?}", err);
                Error::DbAddError.into_err_with_msg("添加DeBox群组成员信息失败")
            })?;

        Ok(result)
    }

    /// 更新DeBox群组成员
    pub async fn update(
        &self,
        ctx: &Context,
        req: UpdateDeboxGroupMemberReq,
    ) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();
        let active_model = debox_group_member::ActiveModel {
            user_id: Set(user_id),
            debox_user_id: Set(req.debox_user_id),
            address: Set(req.address),
            name: Set(req.name),
            pic: Set(req.pic),
            ..Default::default()
        };

        let result = self
            .debox_group_member_dao
            .update(req.id, user_id, active_model)
            .await
            .map_err(|err| {
                error!("更新DeBox群组成员失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox群组成员失败")
            })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(
        &self,
        ctx: &Context,
        req: UpdateDeboxGroupMemberStatusReq,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        self.debox_group_member_dao
            .update_status(req.id, user_id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新DeBox群组成员状态失败, 该DeBox群组成员不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新DeBox群组成员状态失败, 该DeBox群组成员不存在");
                }
                error!("更新DeBox群组成员状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox群组成员状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(
        &self,
        ctx: &Context,
        req: DeleteDeboxGroupMemberReq,
    ) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_group_member_dao
            .delete(req.id, user_id)
            .await
            .map_err(|err| {
                error!("删除DeBox群组成员信息失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除DeBox群组成员信息失败")
            })?;

        Ok(result)
    }
}

impl DeboxGroupMemberService {
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

impl DeboxGroupMemberService {
    /// 同步DeBox群组成员列表
    ///
    /// 该方法会立即返回，实际同步任务会在后台异步执行
    pub async fn sync_group_members(
        &self,
        ctx: &Context,
        req: SyncDeboxGroupMemberReq,
    ) -> Result<(), ErrorMsg> {
        let ctx = ctx.clone();
        let service = self.clone();

        // 在后台异步执行同步任务，不阻塞当前请求
        tokio::spawn(async move {
            if let Err(e) = service.task_sync_group_members(ctx, req).await {
                error!("后台同步DeBox群组成员任务执行失败: {:#?}", e);
            } else {
                info!("后台同步DeBox群组成员任务执行完成");
            }
        });

        Ok(())
    }

    /// 非阻塞线程任务 同步DeBox群组成员列表
    pub async fn task_sync_group_members(
        &self,
        ctx: Context,
        req: SyncDeboxGroupMemberReq,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        // 按账号进行划分群组
        let mut account_group_map: HashMap<i32, Vec<debox_group::Model>> = HashMap::new();
        for group_id in req.group_ids {
            // 获取群组信息
            let group_info = match self.debox_group_dao.info(group_id, user_id).await {
                Ok(group_info) => group_info,
                Err(e) => {
                    error!("查询DeBox群组信息失败, err: {:#?}", e);
                    continue;
                }
            };

            if let Some(group_info) = group_info {
                account_group_map
                    .entry(group_info.account_id)
                    .or_insert(vec![group_info.clone()])
                    .push(group_info);
            }
        }

        for (account_id, groups) in account_group_map.into_iter() {
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

            // 获取群组成员列表
            for group in groups {
                let members = match self.get_all_group_members(&client, &group.gid).await {
                    Ok(members) => members,
                    Err(e) => {
                        error!("获取DeBox群组成员列表失败, err: {:#?}", e);
                        continue;
                    }
                };

                // 批量添加群组成员
                self.batch_add_group_members(user_id, account_id, group, members)
                    .await
                    .map_err(|e| {
                        error!("添加DeBox群组成员信息失败, err: {:#?}", e);
                        Error::DbAddError.into_err_with_msg("添加DeBox群组成员信息失败")
                    })?;
            }
        }

        Ok(())
    }

    /// 批量添加群组成员
    pub async fn batch_add_group_members(
        &self,
        user_id: i32,
        account_id: i32,
        group: debox_group::Model,
        members: Vec<DaoMember>,
    ) -> Result<(), ErrorMsg> {
        let mut active_models = Vec::with_capacity(members.len());
        for member in members {
            active_models.push(debox_group_member::ActiveModel {
                user_id: Set(user_id),
                account_id: Set(account_id),
                group_id: Set(group.id),
                group_gid: Set(group.gid.clone()),
                debox_user_id: Set(member.user_id),
                address: Set(member.address),
                name: Set(member.name),
                pic: Set(Some(member.pic)),
                ..Default::default()
            });
        }

        // 添加成员
        self.debox_group_member_dao
            .creates(active_models)
            .await
            .map_err(|e| {
                error!("添加DeBox群组成员信息失败, err: {:#?}", e);
                Error::DbAddError.into_err_with_msg("添加DeBox群组成员信息失败")
            })?;

        Ok(())
    }

    /// 并发获取所有群组成员列表
    ///
    /// 群组分享链接: https://m.debox.pro/group?id=oes85edz&code=peqt8jxu
    async fn get_all_group_members(
        &self,
        client: &DeBoxClient,
        gid: &str,
    ) -> Result<Vec<DaoMember>, ErrorMsg> {
        // 每页 20 条, 设置仅取前100页的数据
        let total_pages = self.get_group_members_total_pages(client, gid, 0).await?;
        let semaphore = Arc::new(Semaphore::new(CONCURRENCY_LIMIT));

        // 生成所有页的 Future
        let futures = (0..total_pages).map(|page| {
            let semaphore = Arc::clone(&semaphore);
            let group_id = gid.to_string();

            async move {
                let _permit = semaphore.acquire().await.map_err(|e| {
                    error!("获取Semaphore许可失败, err: {:#?}", e);
                    Error::AcquireError(e).into_err_with_msg("获取Semaphore许可失败")
                })?;
                let members = self
                    .get_group_members(client, &group_id, page as u64)
                    .await?;

                Ok::<_, _>(members)
            }
        });

        // 并发执行所有 Future
        let members: Vec<DaoMember> = join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<Vec<DaoMember>>, _>>()?
            .into_iter()
            .flatten()
            .collect();

        Ok(members)
    }

    /// 获取群组成员总分页数
    ///
    /// 每页 20 条
    async fn get_group_members_total_pages(
        &self,
        client: &DeBoxClient,
        gid: &str,
        page: u64,
    ) -> Result<usize, ErrorMsg> {
        let data = DaoMemberReq {
            gid: gid.to_string(),
            page,
            size: 20,
            search: "".to_string(),
            sort_type: 0,
        };
        let resp = client.dao_member(data).await.map_err(|e| {
            error!("获取DeBox群组成员列表失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("获取DeBox群组成员列表失败")
        })?;

        let total_pages = SYNC_PAGE_LIMIT.min(resp.total / 20);

        Ok(total_pages)
    }

    /// 获取群组成员列表
    async fn get_group_members(
        &self,
        client: &DeBoxClient,
        gid: &str,
        page: u64,
    ) -> Result<Vec<DaoMember>, ErrorMsg> {
        let data = DaoMemberReq {
            gid: gid.to_string(),
            page,
            size: 20,
            search: "".to_string(),
            sort_type: 0,
        };
        let resp = client.dao_member(data).await.map_err(|e| {
            error!("获取DeBox群组成员列表失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("获取DeBox群组成员列表失败")
        })?;
        Ok(resp.data)
    }
}
