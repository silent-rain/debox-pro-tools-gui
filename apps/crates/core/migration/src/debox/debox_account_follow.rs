//! DeBox账号关注表
//! Entity: [`entity::user::UserBase`]
//! Entity: [`entity::debox::DeboxAccount`]

use sea_orm::{
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

use crate::{
    debox::debox_account::DeboxAccount, user::user_base::UserBase,
    utils::if_not_exists_create_index,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(DeboxAccountFollow::Table)
                    .comment("账号关注表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeboxAccountFollow::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("关注ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::AccountId)
                            .integer()
                            .not_null()
                            .comment("账号ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::DeboxUserId)
                            .string()
                            .string_len(30)
                            .default("")
                            .comment("DeBox 用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::InviteCode)
                            .string()
                            .string_len(10)
                            .not_null()
                            .default("")
                            .comment("邀请码"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::Avatar)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("账号头像"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::Status)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFollow::UpdatedAt)
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
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DeboxAccountFollow::Table.to_string(),
                                DeboxAccountFollow::UserId.to_string()
                            ))
                            .from_col(DeboxAccountFollow::UserId)
                            .to(UserBase::Table, UserBase::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DeboxAccountFollow::Table.to_string(),
                                DeboxAccountFollow::AccountId.to_string()
                            ))
                            .from_col(DeboxAccountFollow::AccountId)
                            .to(DeboxAccount::Table, DeboxAccount::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // create index
        if_not_exists_create_index(
            manager,
            DeboxAccountFollow::Table,
            vec![DeboxAccountFollow::UserId, DeboxAccountFollow::AccountId],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(DeboxAccountFollow::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DeboxAccountFollow {
    #[sea_orm(iden = "t_debox_account_follow")]
    Table,
    Id,
    UserId,
    AccountId,
    DeboxUserId,
    InviteCode,
    Name,
    Avatar,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
