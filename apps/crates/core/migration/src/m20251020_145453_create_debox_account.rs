//! DeBox账号表
//! Entity: [`entity::prelude::UserAccount`]

use crate::m20240218_145453_create_user_base::UserBase;

use sea_orm::{
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(DeboxAccount::Table)
                    .comment("DeBox账号表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeboxAccount::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("账号ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::UserId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::ApiKey)
                            .string()
                            .string_len(30)
                            .not_null()
                            .comment("开发者 API Key，在DeBox开放平台获取"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::AppSecret)
                            .string()
                            .string_len(30)
                            .not_null()
                            .comment("开发者 App Secret，在DeBox开放平台获取"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::AccessToken)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("登录授权, 有效期较短"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::WebToken)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("WEB登录授权"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::DeboxUserId)
                            .string()
                            .string_len(30)
                            .default("")
                            .comment("DeBox 用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::WalletAddress)
                            .string()
                            .string_len(50)
                            .default("")
                            .comment("用户钱包地址"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::ApiKeyStatus)
                            .boolean()
                            .default(false)
                            .comment("ApiKey 状态(0:无效,1:有效)"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::AccessTokenStatus)
                            .boolean()
                            .default(false)
                            .comment("Access Token 状态(0:无效,1:有效)"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::WebTokenStatus)
                            .boolean()
                            .default(false)
                            .comment("Web Token 状态(0:无效,1:有效)"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::Status)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccount::UpdatedAt)
                            .date_time()
                            .not_null()
                            .extra({
                                match manager.get_database_backend() {
                                    DatabaseBackend::Sqlite => "DEFAULT CURRENT_TIMESTAMP",
                                    _ => "DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP",
                                }
                            })
                            .comment("更新时间"),
                    )
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(DeboxAccount::Table.to_string(), "idx_user_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx_user_id")
                        .table(DeboxAccount::Table)
                        .col(DeboxAccount::UserId)
                        .to_owned(),
                )
                .await?;
        }

        // Sqlite 不支持外键
        if manager.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }

        if !manager
            .has_index(DeboxAccount::Table.to_string(), "fk_user_account_user_id")
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_user_account_user_id")
                        .from(DeboxAccount::Table, DeboxAccount::UserId)
                        .to(UserBase::Table, UserBase::Id)
                        .on_update(ForeignKeyAction::Cascade)
                        .on_delete(ForeignKeyAction::Cascade)
                        .to_owned(),
                )
                .await?;
        }

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(DeboxAccount::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DeboxAccount {
    #[sea_orm(iden = "t_debox_account")]
    Table,
    Id,
    UserId,
    ApiKey,
    AppSecret,
    AccessToken,
    WebToken,
    DeboxUserId,
    WalletAddress,
    ApiKeyStatus,
    AccessTokenStatus,
    WebTokenStatus,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
