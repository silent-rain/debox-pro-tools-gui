//! DeBox群组管理

use log::error;
use nject::injectable;
use sea_orm::{DbErr::RecordNotUpdated, Set};

use entity::debox::debox_group;
use err_code::{Error, ErrorMsg};

use crate::{
    dao::debox_group::DeboxGroupDao,
    dto::debox_group::{
        CreateDeboxGroupReq, DeleteDeboxGroupReq, GetDeboxGroupReq, GetDeboxGroupsReq,
        UpdateDeboxGroupReq, UpdateDeboxGroupStatusReq,
    },
};

/// 服务层
#[injectable]
pub struct DeboxGroupService {
    debox_group_dao: DeboxGroupDao,
}

impl DeboxGroupService {
    /// 获取列表数据
    pub async fn list(
        &self,
        req: GetDeboxGroupsReq,
    ) -> Result<(Vec<debox_group::Model>, u64), ErrorMsg> {
        // 获取所有数据
        if let Some(true) = req.all {
            return self.debox_group_dao.all().await.map_err(|err| {
                error!("查询DeBox群组列表失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox群组列表失败")
            });
        }

        let (results, total) = self.debox_group_dao.list(req).await.map_err(|err| {
            error!("查询DeBox群组列表失败, err: {:#?}", err);
            Error::DbQueryError.into_err_with_msg("查询DeBox群组列表失败")
        })?;

        Ok((results, total))
    }

    /// 获取详情数据
    pub async fn info(&self, req: GetDeboxGroupReq) -> Result<debox_group::Model, ErrorMsg> {
        let result = self
            .debox_group_dao
            .info(req.id)
            .await
            .map_err(|err| {
                error!("查询DeBox群组信息失败, err: {:#?}", err);
                Error::DbQueryError.into_err_with_msg("查询DeBox群组信息失败")
            })?
            .ok_or_else(|| {
                error!("DeBox群组不存在");
                Error::DbQueryEmptyError.into_err_with_msg("DeBox群组不存在")
            })?;

        Ok(result)
    }

    /// 添加数据
    pub async fn create(&self, req: CreateDeboxGroupReq) -> Result<debox_group::Model, ErrorMsg> {
        let model = debox_group::ActiveModel {
            account_id: Set(req.account_id),
            url: Set(req.url),
            group_name: Set(req.group_name),
            group_code: Set(req.group_code),
            desc: Set(req.desc),
            status: Set(true),
            ..Default::default()
        };
        let result = self.debox_group_dao.create(model).await.map_err(|err| {
            error!("添加DeBox群组信息失败, err: {:#?}", err);
            Error::DbAddError.into_err_with_msg("添加DeBox群组信息失败")
        })?;

        Ok(result)
    }

    /// 更新DeBox群组
    pub async fn update(&self, req: UpdateDeboxGroupReq) -> Result<u64, ErrorMsg> {
        let model = debox_group::ActiveModel {
            id: Set(req.id),
            account_id: Set(req.account_id),
            url: Set(req.url),
            group_name: Set(req.group_name),
            group_code: Set(req.group_code),
            desc: Set(req.desc),
            status: Set(req.status),
            ..Default::default()
        };

        let result = self.debox_group_dao.update(model).await.map_err(|err| {
            error!("更新DeBox群组失败, err: {:#?}", err);
            Error::DbUpdateError.into_err_with_msg("更新DeBox群组失败")
        })?;

        Ok(result)
    }

    /// 更新数据状态
    pub async fn update_status(&self, req: UpdateDeboxGroupStatusReq) -> Result<(), ErrorMsg> {
        self.debox_group_dao
            .update_status(req.id, req.status)
            .await
            .map_err(|err| {
                if err == RecordNotUpdated {
                    error!("更新DeBox群组状态失败, 该DeBox群组不存在");
                    return Error::DbUpdateError
                        .into_err_with_msg("更新DeBox群组状态失败, 该DeBox群组不存在");
                }
                error!("更新DeBox群组状态失败, err: {:#?}", err);
                Error::DbUpdateError.into_err_with_msg("更新DeBox群组状态失败")
            })?;

        Ok(())
    }

    /// 删除数据
    pub async fn delete(&self, req: DeleteDeboxGroupReq) -> Result<u64, ErrorMsg> {
        let result = self.debox_group_dao.delete(req.id).await.map_err(|err| {
            error!("删除DeBox群组信息失败, err: {:#?}", err);
            Error::DbDeleteError.into_err_with_msg("删除DeBox群组信息失败")
        })?;

        Ok(result)
    }
}
