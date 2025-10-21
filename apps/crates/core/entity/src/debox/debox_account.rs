//! DeBox账号表

use chrono::Local;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, DbErr, DeriveEntityModel, DerivePrimaryKey, EntityTrait,
    EnumIter, PrimaryKeyTrait, Related, RelationDef, RelationTrait, Set,
    prelude::{DateTime, async_trait::async_trait},
};
use serde::{Deserialize, Serialize};

use crate::user::user_base;

/// DeBox账号表
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "t_debox_account")]
pub struct Model {
    /// 账号ID
    #[sea_orm(primary_key)]
    pub id: i32,
    /// 用户ID
    pub user_id: i32,
    /// 开发者 API Key，在DeBox开放平台获取
    pub api_key: String,
    /// 开发者 App Secret，在DeBox开放平台获取
    pub app_secret: String,
    /// 登录授权, 有效期较短
    pub access_token: String,
    /// WEB登录授权
    pub web_token: String,
    /// DeBox 用户ID
    pub debox_user_id: String,
    /// 用户钱包地址
    pub wallet_address: String,
    /// ApiKey 状态(0:无效,1:有效)
    pub api_key_status: String,
    /// Access Token 状态(0:无效,1:有效)
    pub access_token_status: String,
    /// Web Token 状态(0:无效,1:有效)
    pub web_token_status: String,
    /// 描述信息
    pub desc: Option<String>,
    /// 创建时间
    pub created_at: DateTime,
    /// 更新时间
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    UserBase,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::UserBase => Entity::belongs_to(user_base::Entity)
                .from(Column::UserId)
                .to(user_base::Column::Id)
                .into(),
        }
    }
}

impl Related<user_base::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserBase.def()
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
