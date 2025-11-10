//! DeBox账号关注人管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::debox::{DeboxAccountFollow, debox_account_follow};

use crate::dto::debox_account_follow::{DeboxAccountFollowSort, GetDeboxAccountFollowsReq};

/// 数据访问
#[injectable]
#[derive(Clone)]
pub struct DeboxAccountFollowDao {
    db: Arc<dyn PoolTrait>,
}

impl DeboxAccountFollowDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        user_id: i32,
        req: GetDeboxAccountFollowsReq,
    ) -> Result<(Vec<debox_account_follow::Model>, u64), DbErr> {
        let mut states = DeboxAccountFollow::find()
            .filter(debox_account_follow::Column::UserId.eq(user_id))
            .apply_if(req.start_time, |query, v| {
                query.filter(debox_account_follow::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(debox_account_follow::Column::CreatedAt.lt(v))
            })
            .apply_if(req.status, |query, v| {
                query.filter(debox_account_follow::Column::Status.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        // 排序
        if let Some(sorts) = req.sorts {
            for sort in sorts {
                let DeboxAccountFollowSort(column, order) = sort.try_into()?;
                states = states.order_by(column, order);
            }
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
    ) -> Result<Option<debox_account_follow::Model>, DbErr> {
        DeboxAccountFollow::find_by_id(id)
            .filter(debox_account_follow::Column::UserId.eq(user_id))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: debox_account_follow::ActiveModel,
    ) -> Result<debox_account_follow::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(
        &self,
        id: i32,
        user_id: i32,
        active_model: debox_account_follow::ActiveModel,
    ) -> Result<u64, DbErr> {
        let result = DeboxAccountFollow::update_many()
            .set(active_model)
            .filter(debox_account_follow::Column::Id.eq(id))
            .filter(debox_account_follow::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, user_id: i32, status: bool) -> Result<u64, DbErr> {
        let active_model = debox_account_follow::ActiveModel {
            status: Set(status),
            ..Default::default()
        };

        let result = DeboxAccountFollow::update_many()
            .set(active_model)
            .filter(debox_account_follow::Column::Id.eq(id))
            .filter(debox_account_follow::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32, user_id: i32) -> Result<u64, DbErr> {
        let result = DeboxAccountFollow::delete_by_id(id)
            .filter(debox_account_follow::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl DeboxAccountFollowDao {
    /// 根据debox_user_id获取账号关注信息
    pub async fn follow_by_debox_user_id(
        &self,
        user_id: i32,
        account_id: i32,
        debox_user_id: String,
    ) -> Result<Option<debox_account_follow::Model>, DbErr> {
        let result = DeboxAccountFollow::find()
            .filter(debox_account_follow::Column::UserId.eq(user_id))
            .filter(debox_account_follow::Column::AccountId.eq(account_id))
            .filter(debox_account_follow::Column::DeboxUserId.eq(debox_user_id))
            .one(self.db.db())
            .await?;
        Ok(result)
    }
}
