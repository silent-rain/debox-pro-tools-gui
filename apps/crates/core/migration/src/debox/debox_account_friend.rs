//! DeBox账号好友表
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
                    .table(DeboxAccountFriend::Table)
                    .comment("账号好友表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeboxAccountFriend::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("好友ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::AccountId)
                            .integer()
                            .not_null()
                            .comment("账号ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::DeboxUserId)
                            .string()
                            .string_len(30)
                            .default("")
                            .comment("DeBox 用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("用户名称"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::Avatar)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("账号头像"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::Status)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(DeboxAccountFriend::UpdatedAt)
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
                                DeboxAccountFriend::Table.to_string(),
                                DeboxAccountFriend::UserId.to_string()
                            ))
                            .from_col(DeboxAccountFriend::UserId)
                            .to(UserBase::Table, UserBase::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DeboxAccountFriend::Table.to_string(),
                                DeboxAccountFriend::AccountId.to_string()
                            ))
                            .from_col(DeboxAccountFriend::AccountId)
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
            DeboxAccountFriend::Table,
            vec![DeboxAccountFriend::UserId, DeboxAccountFriend::AccountId],
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(DeboxAccountFriend::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DeboxAccountFriend {
    #[sea_orm(iden = "t_debox_account_friend")]
    Table,
    Id,
    UserId,
    AccountId,
    DeboxUserId,
    Name,
    Avatar,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
