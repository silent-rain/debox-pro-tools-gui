//! 用户区块链钱包管理

use log::error;
use nject::injectable;
use sea_orm::Set;

use entity::user::blockchain_wallet;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::blockchain_wallet::BlockchainWalletDao,
    dto::blockchain_wallet::{
        CreateBlockchainWalletReq, DeleteBlockchainWalletReq, GetBlockchainWalletReq,
        GetBlockchainWalletsReq, UpdateBlockchainWalletReq,
    },
};

/// 服务层
#[injectable]
pub struct BlockchainWalletService {
    blockchain_wallet_dao: BlockchainWalletDao,
}

impl BlockchainWalletService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetBlockchainWalletsReq,
    ) -> Result<(Vec<blockchain_wallet::Model>, u64), ErrorMsg> {
        let (mut results, total) = self.blockchain_wallet_dao.list(req).await.map_err(|err| {
            error!("查询用户区块链钱包列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询用户区块链钱包列表失败")
        })?;

        // 屏蔽敏感信息
        for result in results.iter_mut() {
            result.mnemonic = None;
            result.private_key = None;
        }

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(
        &self,
        req: GetBlockchainWalletReq,
    ) -> Result<blockchain_wallet::Model, ErrorMsg> {
        let mut result = self
            .blockchain_wallet_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询用户区块链钱包信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户区块链钱包信息失败")
            })?
            .ok_or_else(|| {
                error!("用户区块链钱包不存在");
                Error::DbQueryEmptyError.into_err_with_msg("用户区块链钱包不存在")
            })?;

        // 屏蔽敏感信息
        result.mnemonic = None;
        result.private_key = None;
        Ok(result)
    }

    /// 添加数据
    pub async fn create(
        &self,
        req: CreateBlockchainWalletReq,
    ) -> Result<blockchain_wallet::Model, ErrorMsg> {
        // 查询用户区块链钱包是否已存在
        self.check_wallet_address_exist(req.wallet_address.clone(), None)
            .await?;

        let model = blockchain_wallet::ActiveModel {
            user_id: Set(req.user_id),
            wallet_address: Set(req.wallet_address),
            mnemonic: Set(req.mnemonic),
            private_key: Set(req.private_key),
            chain_id: Set(req.chain_id),
            desc: Set(req.desc),
            ..Default::default()
        };
        let result = self
            .blockchain_wallet_dao
            .create(model)
            .await
            .map_err(|err| {
                error!("添加用户区块链钱包信息失败, err: {:#?}", err);
                Error::DbAddError.into_err_with_msg("添加用户区块链钱包信息失败")
            })?;

        Ok(result)
    }

    /// 更新用户区块链钱包
    pub async fn update(&self, req: UpdateBlockchainWalletReq) -> Result<u64, ErrorMsg> {
        let model = blockchain_wallet::ActiveModel {
            id: Set(req.id),
            desc: Set(req.desc),
            ..Default::default()
        };

        let result = self
            .blockchain_wallet_dao
            .update(model)
            .await
            .map_err(|err| {
                error!("更新用户区块链钱包失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新用户区块链钱包失败")
            })?;

        Ok(result)
    }

    /// 检查用户区块链钱包是否存在
    async fn check_wallet_address_exist(
        &self,
        wallet_address: String,
        current_id: Option<i32>,
    ) -> Result<(), ErrorMsg> {
        let result = self
            .blockchain_wallet_dao
            .info_by_wallet_address(wallet_address)
            .await
            .map_err(|err| {
                error!("查询用户区块链钱包信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询用户区块链钱包信息失败")
            })?;

        // 存在
        if let Some(model) = result
            && (current_id.is_none() || Some(model.id) != current_id)
        {
            error!("用户区块链钱包已存在");
            return Err(Error::DbDataExistError.into_err_with_msg("用户区块链钱包已存在"));
        }

        // 不存在
        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteBlockchainWalletReq) -> Result<u64, ErrorMsg> {
        let result = self
            .blockchain_wallet_dao
            .delete(req.id)
            .await
            .map_err(|err| {
                error!("删除用户区块链钱包信息失败, err: {:#?}", err);
                Error::DbDeleteError.into_err_with_msg("删除用户区块链钱包信息失败")
            })?;

        Ok(result)
    }
}
