//! 文件资源管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::system::{FileResourceEntity, file_resource};

use crate::dto::file_resource::GetFileResourcesReq;

/// 数据访问
#[injectable]
pub struct FileResourceDao {
    db: Arc<dyn PoolTrait>,
}

impl FileResourceDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetFileResourcesReq,
    ) -> Result<(Vec<file_resource::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = FileResourceEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(file_resource::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(file_resource::Column::CreatedAt.lt(v))
            })
            .apply_if(req.file_name, |query, v| {
                query.filter(file_resource::Column::FileName.like(format!("{v}%")))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(file_resource::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<file_resource::Model>, DbErr> {
        FileResourceEntity::find()
            .filter(file_resource::Column::Id.eq(id))
            .one(self.db.db())
            .await
    }

    /// 通过hash值获取详情数据
    pub async fn info_by_hash(&self, hash: String) -> Result<Option<file_resource::Model>, DbErr> {
        FileResourceEntity::find()
            .filter(file_resource::Column::Hash.eq(hash))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: file_resource::ActiveModel,
    ) -> Result<file_resource::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 批量添加数据
    pub async fn batch_create(
        &self,
        active_models: Vec<file_resource::ActiveModel>,
    ) -> Result<i32, DbErr> {
        let result = FileResourceEntity::insert_many(active_models)
            .exec(self.db.db())
            .await?;
        Ok(result.last_insert_id)
    }

    /// 更新信息
    pub async fn update(&self, active_model: file_resource::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = FileResourceEntity::update_many()
            .set(active_model)
            .filter(file_resource::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = FileResourceEntity::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }

    /// 按主键批量删除
    pub async fn batch_delete(&self, ids: Vec<i32>) -> Result<u64, DbErr> {
        let result = FileResourceEntity::delete_many()
            .filter(file_resource::Column::Id.is_in(ids))
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
