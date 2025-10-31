//! DeBox账号管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::debox::{DeboxAccountEntity, debox_account};

use crate::dto::debox_account::GetDeboxAccountsReq;

/// 数据访问
#[injectable]
pub struct DeboxAccountDao {
    db: Arc<dyn PoolTrait>,
}

impl DeboxAccountDao {
    /// 获取所有数据
    pub async fn all(
        &self,
        req: GetDeboxAccountsReq,
    ) -> Result<(Vec<debox_account::Model>, u64), DbErr> {
        let results = DeboxAccountEntity::find()
            .apply_if(req.user_id, |query, v| {
                println!("user_id: {}", v);
                query.filter(debox_account::Column::UserId.eq(v))
            })
            .order_by_asc(debox_account::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetDeboxAccountsReq,
    ) -> Result<(Vec<debox_account::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = DeboxAccountEntity::find()
            .filter(debox_account::Column::Status.eq(true))
            .apply_if(req.user_id, |query, v| {
                query.filter(debox_account::Column::UserId.eq(v))
            })
            .apply_if(req.start_time, |query, v| {
                query.filter(debox_account::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(debox_account::Column::CreatedAt.lt(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(debox_account::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<debox_account::Model>, DbErr> {
        DeboxAccountEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: debox_account::ActiveModel,
    ) -> Result<debox_account::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: debox_account::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = DeboxAccountEntity::update_many()
            .set(active_model)
            .filter(debox_account::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, status: bool) -> Result<(), DbErr> {
        let active_model = debox_account::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = DeboxAccountEntity::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl DeboxAccountDao {
    /// 根据user_id和debox_user_id获取账号信息
    pub async fn account_by_user_id_and_debox_user_id(
        &self,
        user_id: i32,
        debox_user_id: String,
    ) -> Result<Option<debox_account::Model>, DbErr> {
        let result = DeboxAccountEntity::find()
            .filter(debox_account::Column::UserId.eq(user_id))
            .filter(debox_account::Column::DeboxUserId.eq(debox_user_id))
            .one(self.db.db())
            .await?;
        Ok(result)
    }

    /// 获取所有的账号
    pub async fn accounts_by_user_id(
        &self,
        user_id: i32,
    ) -> Result<(Vec<debox_account::Model>, u64), DbErr> {
        let states = DeboxAccountEntity::find()
            .filter(debox_account::Column::Status.eq(true))
            .filter(debox_account::Column::UserId.eq(user_id));

        let results = states
            .order_by_desc(debox_account::Column::Id)
            .all(self.db.db())
            .await?;

        let total = results.len() as u64;

        Ok((results, total))
    }
}
