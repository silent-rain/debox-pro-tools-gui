//! DeBox账号好友管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::debox::{DeboxAccountFriend, debox_account_friend};

use crate::dto::debox_account_friend::{DeboxAccountFriendSort, GetDeboxAccountFriendsReq};

/// 数据访问
#[injectable]
#[derive(Clone)]
pub struct DeboxAccountFriendDao {
    db: Arc<dyn PoolTrait>,
}

impl DeboxAccountFriendDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        user_id: i32,
        req: GetDeboxAccountFriendsReq,
    ) -> Result<(Vec<debox_account_friend::Model>, u64), DbErr> {
        let mut states = DeboxAccountFriend::find()
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .apply_if(req.start_time, |query, v| {
                query.filter(debox_account_friend::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(debox_account_friend::Column::CreatedAt.lt(v))
            })
            .apply_if(req.status, |query, v| {
                query.filter(debox_account_friend::Column::Status.eq(v))
            })
            .apply_if(req.account_ids, |query, v| {
                query.filter(debox_account_friend::Column::AccountId.is_in(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        // 排序
        if let Some(sorts) = req.sorts {
            for sort in sorts {
                let DeboxAccountFriendSort(column, order) = sort.try_into()?;
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
    ) -> Result<Option<debox_account_friend::Model>, DbErr> {
        DeboxAccountFriend::find_by_id(id)
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: debox_account_friend::ActiveModel,
    ) -> Result<debox_account_friend::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量插入数据
    pub async fn creates(
        &self,
        active_models: Vec<debox_account_friend::ActiveModel>,
    ) -> Result<(), DbErr> {
        DeboxAccountFriend::insert_many(active_models)
            .exec(self.db.db())
            .await?;
        Ok(())
    }

    /// 更新数据
    pub async fn update(
        &self,
        id: i32,
        user_id: i32,
        active_model: debox_account_friend::ActiveModel,
    ) -> Result<u64, DbErr> {
        let result = DeboxAccountFriend::update_many()
            .set(active_model)
            .filter(debox_account_friend::Column::Id.eq(id))
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, user_id: i32, status: bool) -> Result<u64, DbErr> {
        let active_model = debox_account_friend::ActiveModel {
            status: Set(status),
            ..Default::default()
        };

        let result = DeboxAccountFriend::update_many()
            .set(active_model)
            .filter(debox_account_friend::Column::Id.eq(id))
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32, user_id: i32) -> Result<u64, DbErr> {
        let result = DeboxAccountFriend::delete_by_id(id)
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl DeboxAccountFriendDao {
    /// 根据debox_user_id获取账号好友信息
    pub async fn friend_by_debox_user_id(
        &self,
        user_id: i32,
        account_id: i32,
        debox_user_id: String,
    ) -> Result<Option<debox_account_friend::Model>, DbErr> {
        let result = DeboxAccountFriend::find()
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .filter(debox_account_friend::Column::AccountId.eq(account_id))
            .filter(debox_account_friend::Column::DeboxUserId.eq(debox_user_id))
            .one(self.db.db())
            .await?;
        Ok(result)
    }

    /// 获取指定账号的好友列表
    pub async fn friends_by_account_id(
        &self,
        user_id: i32,
        account_id: i32,
    ) -> Result<Vec<debox_account_friend::Model>, DbErr> {
        let result = DeboxAccountFriend::find()
            .filter(debox_account_friend::Column::Status.eq(true))
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .filter(debox_account_friend::Column::AccountId.eq(account_id))
            .all(self.db.db())
            .await?;
        Ok(result)
    }

    /// 清空指定账号的好友列表
    pub async fn delete_by_account_id(&self, user_id: i32, account_id: i32) -> Result<u64, DbErr> {
        let result = DeboxAccountFriend::delete_many()
            .filter(debox_account_friend::Column::UserId.eq(user_id))
            .filter(debox_account_friend::Column::AccountId.eq(account_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
