//! DeBox群组成员表
//! Entity: [`entity::user::UserBase`]
//! Entity: [`entity::debox::DeboxAccount`]
//! Entity: [`entity::debox::DeboxGroup`]

use sea_orm::{
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

use crate::{
    debox::{debox_account::DeboxAccount, debox_group::DeboxGroup},
    user::user_base::UserBase,
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
                    .table(DeboxGroupMember::Table)
                    .comment("DeBox群组成员表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeboxGroupMember::Id)
                            .integer()
                            .primary_key()
                            .auto_increment()
                            .not_null()
                            .comment("群组ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::UserId)
                            .integer()
                            .not_null()
                            .comment("用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::AccountId)
                            .integer()
                            .not_null()
                            .comment("账号ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::GroupId)
                            .integer()
                            .not_null()
                            .comment("群组ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::GroupGid)
                            .string()
                            .string_len(10)
                            .not_null()
                            .comment("Debox群组ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::DeboxUserId)
                            .integer()
                            .not_null()
                            .comment("Debox用户ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::Address)
                            .string()
                            .string_len(255)
                            .not_null()
                            .comment("钱包地址"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::Name)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("成员名称"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::Pic)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("成员头像"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::IsDangerous)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("是否高危用户,可能导致封号的用户"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::Desc)
                            .string()
                            .string_len(200)
                            .default("")
                            .null()
                            .default("")
                            .comment("描述信息"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::Status)
                            .boolean()
                            .not_null()
                            .default(false)
                            .comment("状态(0:停用,1:正常)"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp())
                            .comment("创建时间"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroupMember::UpdatedAt)
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
                                DeboxGroupMember::Table.to_string(),
                                DeboxGroupMember::UserId.to_string()
                            ))
                            .from_col(DeboxGroupMember::UserId)
                            .to(UserBase::Table, UserBase::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DeboxGroupMember::Table.to_string(),
                                DeboxGroupMember::AccountId.to_string()
                            ))
                            .from_col(DeboxGroupMember::AccountId)
                            .to(DeboxAccount::Table, DeboxAccount::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(format!(
                                "fk_{}_{}",
                                DeboxGroupMember::Table.to_string(),
                                DeboxGroupMember::GroupId.to_string()
                            ))
                            .from_col(DeboxGroupMember::GroupId)
                            .to(DeboxGroup::Table, DeboxGroup::Id)
                            .on_update(ForeignKeyAction::Cascade)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // create index
        if_not_exists_create_index(
            manager,
            DeboxGroupMember::Table,
            vec![DeboxGroupMember::UserId, DeboxGroupMember::AccountId],
        )
        .await?;

        // create unique index
        // if_not_exists_create_unique_index(
        //     manager,
        //     DeboxGroupMember::Table,
        //     vec![
        //         DeboxGroupMember::UserId,
        //         DeboxGroupMember::AccountId,
        //         DeboxGroupMember::GroupId,
        //         DeboxGroupMember::DeboxUserId,
        //     ],
        // )
        // .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_table(Table::drop().table(DeboxGroupMember::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum DeboxGroupMember {
    #[sea_orm(iden = "t_debox_group_member")]
    Table,
    Id,
    UserId,
    AccountId,
    GroupId,
    GroupGid, // 冗余字段，方便查询
    DeboxUserId,
    Address,
    Name,
    Pic,
    // 是否高危用户
    IsDangerous,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
