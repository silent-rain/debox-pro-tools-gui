//! 会员等级管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait, Set,
};

use database::{Pagination, PoolTrait};
use entity::user::{MemberLevelEntity, member_level};

use crate::dto::member_level::GetMemberLevelsReq;

/// 数据访问
#[injectable]
pub struct MemberLevelDao {
    db: Arc<dyn PoolTrait>,
}

impl MemberLevelDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<member_level::Model>, u64), DbErr> {
        let results = MemberLevelEntity::find()
            .order_by_asc(member_level::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetMemberLevelsReq,
    ) -> Result<(Vec<member_level::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = MemberLevelEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(member_level::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(member_level::Column::CreatedAt.lt(v))
            })
            .apply_if(req.name, |query, v| {
                query.filter(member_level::Column::Name.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(member_level::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<member_level::Model>, DbErr> {
        MemberLevelEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 通过名称获取详情信息
    pub async fn info_by_name(&self, name: String) -> Result<Option<member_level::Model>, DbErr> {
        MemberLevelEntity::find()
            .filter(member_level::Column::Name.eq(name))
            .one(self.db.db())
            .await
    }

    /// 通过级别获取详情信息
    pub async fn info_by_level(&self, level: u16) -> Result<Option<member_level::Model>, DbErr> {
        MemberLevelEntity::find()
            .filter(member_level::Column::Level.eq(level))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: member_level::ActiveModel,
    ) -> Result<member_level::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: member_level::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = MemberLevelEntity::update_many()
            .set(active_model)
            .filter(member_level::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, status: bool) -> Result<(), DbErr> {
        let active_model = member_level::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = MemberLevelEntity::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
