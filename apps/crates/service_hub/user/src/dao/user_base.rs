//! 用户信息管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, JoinType,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set, TransactionTrait,
};

use database::{Pagination, PoolTrait};
use entity::user::{RoleEntity, UserBaseEntity, UserRoleRelEntity, role, user_base, user_role_rel};

use crate::dto::user_base::GetUserBasesReq;

/// 数据访问
#[injectable]
pub struct UserBaseDao {
    db: Arc<dyn PoolTrait>,
}

impl UserBaseDao {
    /// 获取所有数据
    pub async fn all(&self) -> Result<(Vec<user_base::Model>, u64), DbErr> {
        let results = UserBaseEntity::find()
            .order_by_asc(user_base::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;
        Ok((results, total))
    }

    /// 获取数据列表
    pub async fn list(&self, req: GetUserBasesReq) -> Result<(Vec<user_base::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = UserBaseEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(user_base::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(user_base::Column::CreatedAt.lt(v))
            })
            .apply_if(req.username, |query, v| {
                query.filter(user_base::Column::Username.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(user_base::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<user_base::Model>, DbErr> {
        UserBaseEntity::find_by_id(id).one(self.db.db()).await
    }

    /// 通过用户名获取详情信息
    pub async fn info_by_username(
        &self,
        username: String,
    ) -> Result<Option<user_base::Model>, DbErr> {
        UserBaseEntity::find()
            .filter(user_base::Column::Username.eq(username))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: user_base::ActiveModel,
    ) -> Result<user_base::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新信息
    pub async fn update(&self, active_model: user_base::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = UserBaseEntity::update_many()
            .set(active_model)
            .filter(user_base::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    // /// 通过分享码获取详情信息
    // pub async fn info_by_share_code(
    //     &self,
    //     share_code: String,
    // ) -> Result<Option<user_base::Model>, DbErr> {
    //     UserBaseEntity::find()
    //         .filter(user_base::Column::ShareCode.eq(share_code))
    //         .one(self.db.db())
    //         .await
    // }

    // /// 更新分享码信息
    // pub async fn update_share_code(&self, id: i32, share_code: String) -> Result<(), DbErr> {
    //     let active_model = user_base::ActiveModel {
    //         id: Set(id),
    //         share_code: Set(Some(share_code)),
    //         ..Default::default()
    //     };
    //     let _ = active_model.update(self.db.db()).await?;
    //     Ok(())
    // }

    /// 更新状态
    pub async fn update_status(&self, id: i32, status: bool) -> Result<(), DbErr> {
        let active_model = user_base::ActiveModel {
            id: Set(id),
            status: Set(status),
            ..Default::default()
        };
        let _ = active_model.update(self.db.db()).await?;
        Ok(())
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = UserBaseEntity::delete_by_id(id).exec(self.db.db()).await?;
        Ok(result.rows_affected)
    }
}

/// 添加/删除用户，同时添加角色
///
/// 事物处理
impl UserBaseDao {
    /// 添加用户及对应用户的角色
    pub async fn add_user(
        &self,
        active_model: user_base::ActiveModel,
        add_role_ids: Vec<i32>,
    ) -> Result<user_base::Model, DbErr> {
        let txn = self.db.db().begin().await?;

        // 添加用户
        let user = self.txn_add_user(&txn, active_model).await?;
        let user_id = user.id;

        // 添加批量角色
        let _ = self
            .txn_batch_add_user_roles(&txn, user_id, add_role_ids)
            .await?;

        txn.commit().await?;
        Ok(user)
    }

    /// 更新用户及对应用户的角色
    pub async fn update_user(
        &self,
        active_model: user_base::ActiveModel,
        add_role_ids: Vec<i32>,
        del_role_ids: Vec<i32>,
    ) -> Result<(), DbErr> {
        let user_id: i32 = *(active_model.id.clone().as_ref());
        let txn = self.db.db().begin().await?;

        // 更新用户
        let _ = self.txn_update_user(&txn, active_model).await?;
        // 添加批量角色
        let _ = self
            .txn_batch_add_user_roles(&txn, user_id, add_role_ids)
            .await?;
        // 删除批量角色
        let _ = self
            .txn_batch_del_user_roles(&txn, user_id, del_role_ids)
            .await?;

        txn.commit().await?;
        Ok(())
    }

    /// 添加用户
    async fn txn_add_user(
        &self,
        txn: &DatabaseTransaction,
        data: user_base::ActiveModel,
    ) -> Result<user_base::Model, DbErr> {
        data.insert(txn).await
    }

    /// 更新用户
    async fn txn_update_user(
        &self,
        txn: &DatabaseTransaction,
        active_model: user_base::ActiveModel,
    ) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = UserBaseEntity::update_many()
            .set(active_model)
            .filter(user_base::Column::Id.eq(id))
            .exec(txn)
            .await?;
        Ok(result.rows_affected)
    }

    /// 批量添加用户的角色
    async fn txn_batch_add_user_roles(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> Result<i32, DbErr> {
        if role_ids.is_empty() {
            return Ok(0);
        }
        let mut user_ids = Vec::new();
        for role_id in role_ids {
            let model = user_role_rel::ActiveModel {
                user_id: Set(user_id),
                role_id: Set(role_id),
                ..Default::default()
            };
            user_ids.push(model)
        }

        let result = UserRoleRelEntity::insert_many(user_ids).exec(txn).await?;
        Ok(result.last_insert_id)
    }

    /// 批量删除用户的角色
    async fn txn_batch_del_user_roles(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        role_ids: Vec<i32>,
    ) -> Result<u64, DbErr> {
        if role_ids.is_empty() {
            return Ok(0);
        }

        let result = UserRoleRelEntity::delete_many()
            .filter(user_role_rel::Column::UserId.eq(user_id))
            .filter(user_role_rel::Column::RoleId.is_in(role_ids))
            .exec(txn)
            .await?;
        Ok(result.rows_affected)
    }
}

impl UserBaseDao {
    /// 通过用户ID获角色色列表
    pub async fn roles(&self, user_id: i32) -> Result<(Vec<role::Model>, u64), DbErr> {
        let results = RoleEntity::find()
            .join_rev(
                JoinType::InnerJoin,
                UserRoleRelEntity::belongs_to(RoleEntity)
                    .from(user_role_rel::Column::RoleId)
                    .to(role::Column::Id)
                    .into(),
            )
            .filter(user_role_rel::Column::UserId.eq(user_id))
            .order_by_asc(role::Column::Id)
            .all(self.db.db())
            .await?;
        let total = results.len() as u64;

        Ok((results, total))
    }
}

#[cfg(test)]
mod tests {
    use sea_orm::DbBackend;

    use super::*;

    #[test]
    fn test_role_list() {
        let result = RoleEntity::find()
            .select_only()
            .columns([role::Column::Id])
            .join_rev(
                JoinType::InnerJoin,
                UserRoleRelEntity::belongs_to(RoleEntity)
                    .from(user_role_rel::Column::RoleId)
                    .to(role::Column::Id)
                    .into(),
            )
            .filter(user_role_rel::Column::UserId.eq(10))
            .order_by_asc(role::Column::Id)
            .build(DbBackend::Postgres)
            .to_string();

        let sql = r#"SELECT "t_user_role"."id" FROM "t_user_role" INNER JOIN "t_user_role_rel" ON "t_user_role_rel"."role_id" = "t_user_role"."id" WHERE "t_user_role_rel"."user_id" = 10 ORDER BY "t_user_role"."id" ASC"#;
        assert_eq!(result, sql);
    }
}
