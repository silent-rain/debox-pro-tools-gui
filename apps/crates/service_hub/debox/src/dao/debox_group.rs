//! DeBox群组管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::debox::{DeboxGroupEntity, debox_group};

use crate::dto::debox_group::GetDeboxGroupsReq;

/// 数据访问
#[injectable]
pub struct DeboxGroupDao {
    db: Arc<dyn PoolTrait>,
}

impl DeboxGroupDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetDeboxGroupsReq,
    ) -> Result<(Vec<debox_group::Model>, u64), DbErr> {
        let page = if req.all.unwrap_or(false) {
            // 如果 `all` 为 true，则不分页
            Pagination::new(1, 1000) // 设置一个非常大的 page_size
        } else {
            // 否则正常分页
            Pagination::new(req.page, req.page_size)
        };

        let states = DeboxGroupEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(debox_group::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(debox_group::Column::CreatedAt.lt(v))
            })
            .apply_if(req.account_ids, |query, v| {
                if v.is_empty() {
                    query // 不添加过滤条件
                } else {
                    query.filter(debox_group::Column::AccountId.is_in(v))
                }
            })
            .apply_if(req.name, |query, v| {
                query.filter(debox_group::Column::Name.like(format!("%{v}%")))
            })
            .apply_if(req.status, |query, v| {
                query.filter(debox_group::Column::Status.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(debox_group::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<debox_group::Model>, DbErr> {
        DeboxGroupEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: debox_group::ActiveModel,
    ) -> Result<debox_group::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: debox_group::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = DeboxGroupEntity::update_many()
            .set(active_model)
            .filter(debox_group::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, status: bool) -> Result<(), DbErr> {
        let active_model = debox_group::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = DeboxGroupEntity::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl DeboxGroupDao {
    /// 根据 account_id 和 gid 获取群组信息
    pub async fn info_by_gid(
        &self,
        account_id: i32,
        gid: String,
    ) -> Result<Option<debox_group::Model>, DbErr> {
        DeboxGroupEntity::find()
            .filter(debox_group::Column::AccountId.eq(account_id))
            .filter(debox_group::Column::Gid.eq(gid))
            .one(self.db.db())
            .await
    }
}
