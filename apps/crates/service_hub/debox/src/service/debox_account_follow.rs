//! DeBox账号关注人管理

use std::time::Duration;

use log::{error, info};
use nject::injectable;
use sea_orm::{ActiveValue::Set, DbErr::RecordNotUpdated};

use axum_context::Context;
use debox_pro_rs::{
    Config as DeBoxConfig, DeBoxClient, UserExtApi,
    dto::user_ext::{
        FollowNewReq, Relation, RelationListReq, RelationStatus, UserSearch, UserSearchReq,
    },
};
use entity::debox::{debox_account, debox_account_follow};
use err_code::{Error, ErrorMsg};
use tokio::time::sleep;

use crate::{
    DeboxAccountDao, DeboxAccountFollowDao, DeboxGroupMemberDao,
    dto::debox_account_follow::{
        BatchAccountFollowsReq, CreateDeboxAccountFollowReq, DeboxUserSearchReq,
        DeleteDeboxAccountFollowReq, GetDeboxAccountFollowReq, GetDeboxAccountFollowsReq,
        SyncDeboxAccountFollowsReq, UpdateDeboxAccountFollowReq, UpdateDeboxAccountFollowStatusReq,
    },
    enums::debox_account_follow::FollowType,
    utils::extract_url_params,
};

/// 服务层
#[injectable]
pub struct DeboxAccountFollowService {
    debox_account_follow_dao: DeboxAccountFollowDao,
    debox_account_dao: DeboxAccountDao,
    debox_group_member_dao: DeboxGroupMemberDao,
}

impl DeboxAccountFollowService {
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

    /// 关注/粉丝/好友列表
    async fn relation_list(
        &self,
        client: &DeBoxClient,
        page: u64,
        status: RelationStatus,
        look_user_id: Option<u64>,
    ) -> Result<Vec<Relation>, ErrorMsg> {
        let data = RelationListReq {
            page,
            size: 20,
            status,
            look_user_id,
        };

        UserExtApi::relation_list(client, data).await.map_err(|e| {
            error!("获取 DeBox 账号信息失败, err: {:#?}", e);
            Error::DeboxProRs(e).into_err_with_msg("获取 DeBox 账号信息失败")
        })
    }

    /// 关注账号
    async fn debox_follow_account(
        &self,
        client: &DeBoxClient,
        follow_id: &str,
    ) -> Result<(), ErrorMsg> {
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
}

impl DeboxAccountFollowService {
    /// 获取列表数据
    pub async fn list(
        &self,
        ctx: &Context,
        req: GetDeboxAccountFollowsReq,
    ) -> Result<(Vec<debox_account_follow::Model>, u64), ErrorMsg> {
        let user_id = ctx.get_user_id();

        if let Some(account_ids) = &req.account_ids
            && account_ids.is_empty()
        {
            error!("请添加至少一个账号进行查询");
            return Err(
                Error::InvalidParameter("请添加至少一个账号进行查询".to_string()).into_err(),
            );
        }

        let (results, total) = self
            .debox_account_follow_dao
            .list(user_id, req)
            .await
            .map_err(|err| {
                error!("查询DeBox账号关注人列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号关注人列表失败")
            })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        ctx: &Context,
        req: GetDeboxAccountFollowReq,
    ) -> Result<debox_account_follow::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_account_follow_dao
            .info(req.id, user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号关注人信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号关注人信息失败")
            })?
            .ok_or_else(|| {
                error!("DeBox账号关注人不存在");
                Error::DbQueryEmptyError.into_err_with_msg("DeBox账号关注人不存在")
            })?;

        Ok(result)
    }

    // 是否存在 DeBox 账号
    async fn exist_debox_user_id(
        &self,
        user_id: i32,
        account_id: i32,
        debox_user_id: String,
    ) -> Result<bool, ErrorMsg> {
        let result = self
            .debox_account_follow_dao
            .follow_by_debox_user_id(user_id, account_id, debox_user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号关注人信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号关注人信息失败")
            })?
            .is_some();

        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        ctx: &Context,
        req: CreateDeboxAccountFollowReq,
    ) -> Result<debox_account_follow::Model, ErrorMsg> {
        let user_id = ctx.get_user_id();

        // 是否存在 DeBox 账号
        if self
            .exist_debox_user_id(user_id, req.account_id, req.debox_user_id.clone())
            .await?
        {
            error!(
                "DeBox账号关注人已存在, user_id: {:#?}, debox_user_id: {:#?}",
                user_id, req.debox_user_id
            );
            return Err(Error::DbDataExistError.into_err_with_msg("DeBox账号关注人已存在"));
        }

        // 创建基础模型
        let active_model = debox_account_follow::ActiveModel {
            user_id: Set(user_id),
            account_id: Set(req.account_id),
            debox_user_id: Set(req.debox_user_id.clone()),
            invite_code: Set(req.invite_code.clone()),
            name: Set(req.name.clone()),
            avatar: Set(req.avatar.clone()),
            desc: Set(req.desc.clone()),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self
            .debox_account_follow_dao
            .create(active_model)
            .await
            .map_err(|err| {
                error!("添加DeBox账号关注人信息失败, err: {:#?}", err);
                Error::DbAddError.into_err_with_msg("添加DeBox账号关注人信息失败")
            })?;

        Ok(result)
    }

    /// 更新DeBox账号关注人
    pub async fn update(
        &self,
        ctx: &Context,
        req: UpdateDeboxAccountFollowReq,
    ) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();
        let id = req.id;

        let active_model = debox_account_follow::ActiveModel {
            name: Set(req.name.clone()),
            avatar: Set(req.avatar.clone()),
            desc: Set(req.desc.clone()),
            status: Set(req.status),
            ..Default::default()
        };

        self.debox_account_follow_dao
            .update(id, user_id, active_model)
            .await
            .map_err(|err| {
                error!("更新DeBox账号关注人失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox账号关注人失败")
            })
    }

    /// 更新数据状态
    pub async fn update_status(
        &self,
        ctx: &Context,
        req: UpdateDeboxAccountFollowStatusReq,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        self.debox_account_follow_dao
            .update_status(req.id, user_id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新DeBox账号关注人状态失败, 该DeBox账号关注人不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新DeBox账号关注人状态失败, 该DeBox账号关注人不存在");
                }
                error!("更新DeBox账号关注人状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox账号关注人状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(
        &self,
        ctx: &Context,
        req: DeleteDeboxAccountFollowReq,
    ) -> Result<u64, ErrorMsg> {
        let user_id = ctx.get_user_id();

        let result = self
            .debox_account_follow_dao
            .delete(req.id, user_id)
            .await
            .map_err(|err| {
                error!("删除DeBox账号关注人信息失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除DeBox账号关注人信息失败")
            })?;

        Ok(result)
    }
}

impl DeboxAccountFollowService {
    /// 同步关注人列表
    pub async fn sync_follows(
        &self,
        ctx: &Context,
        req: SyncDeboxAccountFollowsReq,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        for account_id in req.account_ids.into_iter() {
            // 获取账号信息
            let account = self.get_account(user_id, account_id).await?;

            // 获取DeBox客户端
            let client = self.debox_client(&account)?;

            // 获取所有关注人
            self.get_all_follows(&client, user_id, account_id).await?;
        }
        Ok(())
    }

    /// 获取账号信息
    async fn get_account(
        &self,
        user_id: i32,
        account_id: i32,
    ) -> Result<debox_account::Model, ErrorMsg> {
        let result = self
            .debox_account_dao
            .info(account_id, user_id)
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

    /// 获取所有关注人
    async fn get_all_follows(
        &self,
        client: &DeBoxClient,
        user_id: i32,
        account_id: i32,
    ) -> Result<(), ErrorMsg> {
        // 失败停止次数
        let max_fail_count = 3;
        let mut fail_count = 0;

        let mut page = 1;
        loop {
            // 分页获取关注人
            let follows = match self
                .relation_list(client, page, RelationStatus::Follow, None)
                .await
            {
                Ok(v) => v,
                Err(e) => {
                    error!("account_id: {} page: {}, err: {:#?}", account_id, page, e);
                    fail_count += 1;
                    if fail_count >= max_fail_count {
                        error!(
                            "account_id: {} page: {} fail_count: {}, 失败次数超过最大次数",
                            account_id, page, fail_count
                        );
                        return Err(e);
                    }
                    break;
                }
            };

            if follows.is_empty() {
                break;
            }
            page += 1;

            // 批量添加关注人
            self.batch_create(follows, user_id, account_id).await?;

            sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }

    /// 批量添加关注人
    async fn batch_create(
        &self,
        follows: Vec<Relation>,
        user_id: i32,
        account_id: i32,
    ) -> Result<(), ErrorMsg> {
        let mut active_models = Vec::new();

        for follow in follows {
            // https://m.debox.pro/card?id=7d2jypx2\u0026invite_code=
            let invite_code = extract_url_params(&follow.url, "id")
                .map_err(|e| {
                    error!("提取URL参数失败, err: {:#?}", e);
                    e.into_err_with_msg("提取URL参数失败")
                })?
                .ok_or_else(|| {
                    error!("id not found");
                    Error::InvalidUrlParameter("id not found".to_string())
                        .into_err_with_msg("id not found")
                })?;

            let active_model = debox_account_follow::ActiveModel {
                user_id: Set(user_id),
                account_id: Set(account_id),
                debox_user_id: Set(follow.user_id.to_string()),
                invite_code: Set(invite_code),
                name: Set(follow.name),
                avatar: Set(Some(follow.pic)),
                status: Set(true),
                ..Default::default()
            };
            active_models.push(active_model);
        }
        // 删除指定账号的关注人列表
        let _result = self
            .debox_account_follow_dao
            .delete_by_account_id(user_id, account_id)
            .await
            .map_err(|err| {
                error!("删除DeBox账号关注人失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除DeBox账号关注人失败")
            })?;

        // 批量插入关注人列表
        self.debox_account_follow_dao
            .creates(active_models)
            .await
            .map_err(|err| {
                error!("批量添加DeBox账号关注人失败, err: {:#?}", err);
                Error::DbBatchAddError.into_err_with_msg("批量添加DeBox账号关注人失败")
            })?;

        Ok(())
    }
}

impl DeboxAccountFollowService {
    /// 批量关注用户
    pub async fn batch_follows(
        &self,
        ctx: &Context,
        req: BatchAccountFollowsReq,
    ) -> Result<(), ErrorMsg> {
        let user_id = ctx.get_user_id();

        // 获取账号信息
        let account = self.get_account(user_id, req.account_id).await?;

        // 获取DeBox客户端
        let client = self.debox_client(&account)?;

        // 当前账号已关注的用户ID列表
        let followed_ids = self.get_follows_by_account(user_id, req.account_id).await?;

        let to_followed_ids = match req.follow_type {
            FollowType::Account => {
                // 目标账号的关注人列表
                let target_followed_ids = self
                    .get_follows_by_account(user_id, req.target_account_id)
                    .await?;

                // 待关注用户ID列表 = 目标账号的关注人列表 - 当前账号已关注的用户ID列表
                let to_followed_ids: Vec<String> = target_followed_ids
                    .into_iter()
                    .filter(|id| !followed_ids.contains(id))
                    .collect();

                to_followed_ids
            }
            FollowType::Group => {
                // 获取账号群组成员列表
                let group_members = self
                    .get_group_members_by_target_group(
                        user_id,
                        req.target_account_id,
                        req.target_group_id,
                    )
                    .await?;

                // 待关注用户ID列表 = 账号群组成员列表 - 当前账号已关注的用户ID列表
                let to_followed_ids: Vec<String> = group_members
                    .into_iter()
                    .filter(|id| !followed_ids.contains(id))
                    .collect();
                to_followed_ids
            }
            FollowType::User => {
                // 待关注用户ID列表 = 请求体中的用户ID列表 - 当前账号已关注的用户ID列表
                let to_followed_ids: Vec<String> = req
                    .debox_user_ids
                    .into_iter()
                    .filter(|id| !followed_ids.contains(id))
                    .collect();
                to_followed_ids
            }
        };

        info!("当前账号已关注的用户ID数: {:#?}", followed_ids.len());
        info!("待关注人数: {:#?}", to_followed_ids.len());

        if to_followed_ids.is_empty() {
            info!("没有需要关注的用户");
            return Ok(());
        }

        // 批量添加关注人
        for follow_id in to_followed_ids.iter() {
            // 关注账号
            self.debox_follow_account(&client, follow_id).await?;
            sleep(Duration::from_millis(500)).await;
        }

        // 同步账号
        self.sync_follows(
            ctx,
            SyncDeboxAccountFollowsReq {
                account_ids: vec![req.account_id],
            },
        )
        .await?;
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
                error!("查询DeBox账号关注人失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号关注人失败")
            })?;

        let followed_ids: Vec<String> = followeds
            .into_iter()
            .map(|followed| followed.debox_user_id)
            .collect();

        Ok(followed_ids)
    }

    /// 获取账号群组成员列表
    async fn get_group_members_by_target_group(
        &self,
        user_id: i32,
        account_id: i32,
        group_id: i32,
    ) -> Result<Vec<String>, ErrorMsg> {
        let members = self
            .debox_group_member_dao
            .list_by_group(user_id, account_id, group_id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号群组成员失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号群组成员失败")
            })?;
        let member_ids: Vec<String> = members
            .into_iter()
            .map(|member| member.debox_user_id.to_string())
            .collect();
        Ok(member_ids)
    }
}

impl DeboxAccountFollowService {
    /// 用户搜索
    pub async fn debox_user_search(
        &self,
        ctx: &Context,
        req: DeboxUserSearchReq,
    ) -> Result<Vec<UserSearch>, ErrorMsg> {
        let user_id = ctx.get_user_id();
        let account = self
            .debox_account_dao
            .info(req.account_id, user_id)
            .await
            .map_err(|err| {
                error!("查询DeBox账号信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox账号信息失败")
            })?
            .ok_or_else(|| {
                error!("DeBox账号不存在");
                Error::DbQueryEmptyError.into_err_with_msg("DeBox账号不存在")
            })?;

        let client = self.debox_client(&account)?;

        let data = UserSearchReq {
            search: req.search,
            page: req.page,
            size: req.size,
        };
        let resp = client.user_search(data).await.map_err(|err| {
            error!("DeBox用户搜索失败, err: {:#?}", err);
            Error::DeboxProRs(err).into_err_with_msg("DeBox用户搜索失败")
        })?;
        Ok(resp)
    }
}
