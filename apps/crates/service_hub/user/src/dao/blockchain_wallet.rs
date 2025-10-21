//! 用户区块链钱包管理
use std::sync::Arc;

use nject::injectable;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, QueryTrait,
};

use database::{Pagination, PoolTrait};
use entity::user::{BlockchainWalletEntity, blockchain_wallet};

use crate::dto::blockchain_wallet::GetBlockchainWalletsReq;

/// 数据访问
#[injectable]
pub struct BlockchainWalletDao {
    db: Arc<dyn PoolTrait>,
}

impl BlockchainWalletDao {
    /// 获取数据列表
    pub async fn list(
        &self,
        req: GetBlockchainWalletsReq,
    ) -> Result<(Vec<blockchain_wallet::Model>, u64), DbErr> {
        let page = Pagination::new(req.page, req.page_size);

        let states = BlockchainWalletEntity::find()
            .apply_if(req.start_time, |query, v| {
                query.filter(blockchain_wallet::Column::CreatedAt.gte(v))
            })
            .apply_if(req.end_time, |query, v| {
                query.filter(blockchain_wallet::Column::CreatedAt.lt(v))
            })
            .apply_if(req.user_id, |query, v| {
                query.filter(blockchain_wallet::Column::UserId.eq(v))
            })
            .apply_if(req.wallet_address, |query, v| {
                query.filter(blockchain_wallet::Column::WalletAddress.eq(v))
            });

        let total = states.clone().count(self.db.db()).await?;
        if total == 0 {
            return Ok((vec![], total));
        }

        let results = states
            .order_by_desc(blockchain_wallet::Column::Id)
            .offset(page.offset())
            .limit(page.page_size())
            .all(self.db.db())
            .await?;

        Ok((results, total))
    }

    /// 获取详情信息
    pub async fn info(&self, id: i32) -> Result<Option<blockchain_wallet::Model>, DbErr> {
        BlockchainWalletEntity::find_by_id(id)
            .one(self.db.db())
            .await
    }

    /// 通过钱包地址获取详情信息
    pub async fn info_by_wallet_address(
        &self,
        wallet_address: String,
    ) -> Result<Option<blockchain_wallet::Model>, DbErr> {
        BlockchainWalletEntity::find()
            .filter(blockchain_wallet::Column::WalletAddress.eq(wallet_address))
            .one(self.db.db())
            .await
    }

    /// 添加详情信息
    pub async fn create(
        &self,
        active_model: blockchain_wallet::ActiveModel,
    ) -> Result<blockchain_wallet::Model, DbErr> {
        active_model.insert(self.db.db()).await
    }

    /// 更新数据
    pub async fn update(&self, active_model: blockchain_wallet::ActiveModel) -> Result<u64, DbErr> {
        let id: i32 = *(active_model.id.clone().as_ref());
        let result = BlockchainWalletEntity::update_many()
            .set(active_model)
            .filter(blockchain_wallet::Column::Id.eq(id))
            .exec(self.db.db())
            .await?;

        Ok(result.rows_affected)
    }

    /// 按主键删除信息
    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        let result = BlockchainWalletEntity::delete_by_id(id)
            .exec(self.db.db())
            .await?;
        Ok(result.rows_affected)
    }
}
