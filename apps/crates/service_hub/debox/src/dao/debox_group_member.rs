//! DeBox群组成员管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::debox::{DeboxGroupMember, debox_group_member};

use crate::dto::debox_group_member::GetDeboxGroupMembersReq;

/// 数据访问
#[injectable]
#[derive(Clone)]
pub struct DeboxGroupMemberDao {
    db: Arc<dyn PoolTrait>,
}

impl DeboxGroupMemberDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        user_id: i32,
        req: GetDeboxGroupMembersReq,
    ) -> Result<(Vec<debox_group_member::Model>, u64), DbErr> {
        let mut states = DeboxGroupMember::find()
            .filter(debox_group_member::Column::UserId.eq(user_id))
            .apply_if(req.start_time, |query, v| {
                query.filter(debox_group_member::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(debox_group_member::Column::CreatedAt.lt(v))
            })
            .apply_if(req.account_ids, |query, v| {
                if v.is_empty() {
                    query // 不添加过滤条件
                } else {
                    query.filter(debox_group_member::Column::AccountId.is_in(v))
                }
            })
            .apply_if(req.group_ids, |query, v| {
                if v.is_empty() {
                    query // 不添加过滤条件
                } else {
                    query.filter(debox_group_member::Column::GroupId.is_in(v))
                }
            })
            .apply_if(req.name, |query, v| {
                query.filter(debox_group_member::Column::Name.like(format!("%{v}%")))
            })
            .apply_if(req.status, |query, v| {
                query.filter(debox_group_member::Column::Status.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        // 分页处理
        if !req.all.unwrap_or(false) {
            let page = Pagination::new(req.page, req.page_size);
            states = states.offset(page.offset()).limit(page.page_size());
        }

        let results = states.all(self.db.db()).await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(
        &self,
        id: i32,
        user_id: i32,
    ) -> Result<Option<debox_group_member::Model>, DbErr> {
        DeboxGroupMember::find_by_id(id)
            .filter(debox_group_member::Column::UserId.eq(user_id))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: debox_group_member::ActiveModel,
    ) -> Result<debox_group_member::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量添加详情信息
    pub async fn creates(
        &self,
        active_models: Vec<debox_group_member::ActiveModel>,
    ) -> Result<i32, DbErr> {
        if active_models.is_empty() {
            return Ok(0);
        }
        let result = DeboxGroupMember::insert_many(active_models)
            .exec(self.db.db())
            .await?;

        Ok(result.last_insert_id)
    }

    /// 更新数据
    pub async fn update(
        &self,
        id: i32,
        user_id: i32,
        active_model: debox_group_member::ActiveModel,
    ) -> Result<u64, DbErr> {
        let result = DeboxGroupMember::update_many()
            .set(active_model)
            .filter(debox_group_member::Column::Id.eq(id))
            .filter(debox_group_member::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, user_id: i32, status: bool) -> Result<u64, DbErr> {
        let active_model = debox_group_member::ActiveModel {
            status: Set(status),
            ..Default::default()
        };

        let result = DeboxGroupMember::update_many()
            .set(active_model)
            .filter(debox_group_member::Column::Id.eq(id))
            .filter(debox_group_member::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32, user_id: i32) -> Result<u64, DbErr> {
        let result = DeboxGroupMember::delete_by_id(id)
            .filter(debox_group_member::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl DeboxGroupMemberDao {
    /// 根据 user_id, account_id, group_gid, debox_user_id 获取群组成员信息
    pub async fn info_by_group_member(
        &self,
        user_id: i32,
        account_id: i32,
        group_gid: String,
        debox_user_id: String,
    ) -> Result<Option<debox_group_member::Model>, DbErr> {
        DeboxGroupMember::find()
            .filter(debox_group_member::Column::UserId.eq(user_id))
            .filter(debox_group_member::Column::AccountId.eq(account_id))
            .filter(debox_group_member::Column::GroupGid.eq(group_gid))
            .filter(debox_group_member::Column::DeboxUserId.eq(debox_user_id))
            .one(self.db.db())
            .await
    }
}
