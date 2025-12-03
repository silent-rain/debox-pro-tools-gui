//! DeBox群组表
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
                    .table(DeboxGroup::Table)
                    .comment("DeBox群组表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeboxGroup::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("群组ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::AccountId)
                            .integer()
                            .not_null()
                            .comment("账号ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::Gid)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("群组ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("群组名称"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::InviteCode)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("群组邀请码"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::Pic)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("群头像"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::Status)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("状态(false:停用,true:正常)"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::UpdatedAt)
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
                                DeboxGroup::Table.to_string(),
                                DeboxGroup::UserId.to_string()
                            ))
                            .from_col(DeboxGroup::UserId)
                            .to(UserBase::Table, UserBase::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DeboxGroup::Table.to_string(),
                                DeboxGroup::AccountId.to_string()
                            ))
                            .from_col(DeboxGroup::AccountId)
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
            DeboxGroup::Table,
            vec![DeboxGroup::UserId, DeboxGroup::AccountId],
        )
        .await?;

        // create unique index
        // if_not_exists_create_unique_index(
        //     manager,
        //     DeboxGroup::Table,
        //     vec![DeboxGroup::UserId, DeboxGroup::AccountId, DeboxGroup::Gid],
        // )
        // .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(DeboxGroup::Table).to_owned())
            .await
    }
}

// 群组分享链接: "https://m.debox.pro/group?id=l3izdfzd&code=2y9u8fkw",
#[derive(DeriveIden)]
pub enum DeboxGroup {
    #[sea_orm(iden = "t_debox_group")]
    Table,
    Id,
    UserId,
    AccountId,
    Gid,
    Name,
    InviteCode,
    Pic,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
