//! DeBox群组表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};

use crate::debox::debox_account;

/// DeBox群组表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_debox_group")]
pub struct Model {
    /// 群组ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 账号ID
    pub account_id: i32,
    /// 群组分享链接
    pub url: String,
    /// 群组名称
    pub group_name: String,
    /// 群组邀请码
    pub group_code: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 状态(false:停用,true:正常)
    pub status: bool,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    DeboxGroup,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::DeboxGroup => Entity::belongs_to(debox_account::Entity)
                .from(Column::AccountId)
                .to(debox_account::Column::Id)
                .into(),
        }
    }
}

impl Related<debox_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeboxGroup.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// Will be triggered before insert / update
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        self.updated_at = Set(Local::now().naive_local());
        Ok(self)
    }
}
