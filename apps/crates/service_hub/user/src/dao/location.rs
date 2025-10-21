//! 用户地理位置管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::user::{LocationEntity, location};

use crate::dto::location::GetLocationsReq;

/// 数据访问
#[injectable]
pub struct LocationDao {
    db: Arc<dyn PoolTrait>,
}

impl LocationDao {
    /// 获取数据列表
    pub async fn list(&self, req: GetLocationsReq) -> Result<(Vec<location::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = LocationEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(location::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(location::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(location::Column::UserId.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(location::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<location::Model>, DbErr> {
        LocationEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 通过用户ID获取详情信息
    pub async fn info_user_id(&self, user_id: i32) -> Result<Option<location::Model>, DbErr> {
        LocationEntity::find()
            .filter(location::Column::UserId.eq(user_id))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: location::ActiveModel,
    ) -> Result<location::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: location::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = LocationEntity::update_many()
            .set(active_model)
            .filter(location::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = LocationEntity::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}
