//! DeBox群组成员表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};

use crate::{
    debox::{debox_account, debox_group},
    user::user_base,
};

#[derive(Clone, Debug, Default, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "t_debox_group_member")]
pub struct Model {
    /// 群组成员ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 账号ID
    pub account_id: i32,
    /// 群组ID
    pub group_id: i32,
    /// Debox群组ID
    pub group_gid: String,
    /// Debox用户ID
    pub debox_user_id: u64,
    /// 钱包地址
    pub address: String,
    /// 成员名称
    pub name: String,
    /// 成员头像
    pub pic: Option<String>,
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
    UserBase,
    DeboxAccount,
    DeboxGroup,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserBase => Entity::belongs_to(user_base::Entity)
                .from(Column::UserId)
                .to(user_base::Column::Id)
                .into(),
            Self::DeboxAccount => Entity::belongs_to(debox_account::Entity)
                .from(Column::AccountId)
                .to(debox_account::Column::Id)
                .into(),
            Self::DeboxGroup => Entity::belongs_to(debox_group::Entity)
                .from(Column::GroupId)
                .to(debox_group::Column::Id)
                .into(),
        }
    }
}

impl Related<user_base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBase.def()
    }
}

impl Related<debox_account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DeboxAccount.def()
    }
}

impl Related<debox_group::Entity> for Entity {
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
