//! DeBox账号管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::debox::{DeboxAccount, debox_account};

use crate::dto::debox_account::{DeboxAccountSort, GetDeboxAccountsReq};

/// 数据访问
#[injectable]
#[derive(Clone)]
pub struct DeboxAccountDao {
    db: Arc<dyn PoolTrait>,
}

impl DeboxAccountDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        user_id: i32,
        req: GetDeboxAccountsReq,
    ) -> Result<(Vec<debox_account::Model>, u64), DbErr> {
        let mut states = DeboxAccount::find()
            .filter(debox_account::Column::UserId.eq(user_id))
            .apply_if(req.start_time, |query, v| {
                query.filter(debox_account::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(debox_account::Column::CreatedAt.lt(v))
            })
            .apply_if(req.status, |query, v| {
                query.filter(debox_account::Column::Status.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        // 排序
        if let Some(sorts) = req.sorts {
            for sort in sorts {
                let DeboxAccountSort(column, order) = sort.try_into()?;
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
    pub async fn info(&self, id: i32, user_id: i32) -> Result<Option<debox_account::Model>, DbErr> {
        DeboxAccount::find_by_id(id)
            .filter(debox_account::Column::UserId.eq(user_id))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: debox_account::ActiveModel,
    ) -> Result<debox_account::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(
        &self,
        id: i32,
        user_id: i32,
        active_model: debox_account::ActiveModel,
    ) -> Result<u64, DbErr> {
        let result = DeboxAccount::update_many()
            .set(active_model)
            .filter(debox_account::Column::Id.eq(id))
            .filter(debox_account::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 更新状态
    pub async fn update_status(&self, id: i32, user_id: i32, status: bool) -> Result<u64, DbErr> {
        let active_model = debox_account::ActiveModel {
            status: Set(status),
            ..Default::default()
        };

        let result = DeboxAccount::update_many()
            .set(active_model)
            .filter(debox_account::Column::Id.eq(id))
            .filter(debox_account::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32, user_id: i32) -> Result<u64, DbErr> {
        let result = DeboxAccount::delete_by_id(id)
            .filter(debox_account::Column::UserId.eq(user_id))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}

impl DeboxAccountDao {
    /// 根据debox_user_id获取账号信息
    pub async fn account_by_debox_user_id(
        &self,
        user_id: i32,
        debox_user_id: String,
    ) -> Result<Option<debox_account::Model>, DbErr> {
        let result = DeboxAccount::find()
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
        let results = DeboxAccount::find()
            .filter(debox_account::Column::Status.eq(true))
            .filter(debox_account::Column::UserId.eq(user_id))
            .order_by_desc(debox_account::Column::Id)
            .all(self.db.db())
            .await?;

        let total = results.len() as u64;

        Ok((results, total))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use sea_orm::{
        ActiveValue::Set, ColumnTrait, DbBackend, EntityTrait, IntoActiveModel, QueryFilter,
        QuerySelect, QueryTrait,
    };

    #[test]
    fn test_into_active_model() {
        let model = debox_account::Model {
            name: "f7641fa0-dr".to_string(),
            status: true,
            ..Default::default()
        };
        let mut active_model = model.into_active_model();
        active_model.name = Set("sr".to_string());

        let id: i32 = 3;

        let result = DeboxAccount::update_many()
            .set(active_model)
            .filter(debox_account::Column::Id.eq(id))
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"UPDATE `t_debox_account` SET `name` = 'sr' WHERE `t_debox_account`.`id` = 3"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_find_by_id() {
        let id = 3;
        let user_id = 1;

        let result = DeboxAccount::find_by_id(id)
            .filter(debox_account::Column::UserId.eq(user_id))
            .select_only()
            .columns([
                debox_account::Column::Id,
                debox_account::Column::UserId,
                debox_account::Column::Name,
            ])
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `t_debox_account`.`id`, `t_debox_account`.`user_id`, `t_debox_account`.`name` FROM `t_debox_account` WHERE `t_debox_account`.`id` = 3 AND `t_debox_account`.`user_id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_delete_by_id() {
        let id = 3;
        let user_id = 1;

        let result = DeboxAccount::delete_by_id(id)
            .filter(debox_account::Column::UserId.eq(user_id))
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"DELETE FROM `t_debox_account` WHERE `t_debox_account`.`id` = 3 AND `t_debox_account`.`user_id` = 1"#;

        assert_eq!(result, sql);
    }

    #[test]
    fn test_order_by_desc() {
        let result = DeboxAccount::find()
            .select_only()
            .columns([
                debox_account::Column::Id,
                debox_account::Column::UserId,
                debox_account::Column::Name,
            ])
            .order_by_desc(debox_account::Column::Id)
            .order_by_asc(debox_account::Column::UserId)
            .build(DbBackend::MySql)
            .to_string();

        let sql = r#"SELECT `t_debox_account`.`id`, `t_debox_account`.`user_id`, `t_debox_account`.`name` FROM `t_debox_account` ORDER BY `t_debox_account`.`id` DESC, `t_debox_account`.`user_id` ASC"#;

        assert_eq!(result, sql);
    }
}
