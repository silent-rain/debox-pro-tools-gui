//! DeBox群组表
//! Entity: [`entity::prelude::DeboxAccount`]

use sea_orm::{
    DatabaseBackend, DeriveIden, DeriveMigrationName, Iden,
    sea_query::{ColumnDef, Expr, ForeignKey, ForeignKeyAction, Index, Table},
};
use sea_orm_migration::{DbErr, MigrationTrait, SchemaManager, async_trait};

use crate::debox::debox_account::DeboxAccount;

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
                            .string()
                            .string_len(20)
                            .primary_key()
                            .not_null()
                            .comment("群组ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::AccountId)
                            .integer()
                            .unique_key()
                            .not_null()
                            .comment("账号ID"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::Url)
                            .string()
                            .string_len(60)
                            .not_null()
                            .comment("群组分享链接"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::GroupName)
                            .string()
                            .string_len(50)
                            .not_null()
                            .comment("群组名称"),
                    )
                    .col(
                        ColumnDef::new(DeboxGroup::GroupCode)
                            .string()
                            .string_len(250)
                            .default("")
                            .comment("群组邀请码"),
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
                            .comment("状态(0:停用,1:正常)"),
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
                    .to_owned(),
            )
            .await?;

        if !manager
            .has_index(DeboxGroup::Table.to_string(), "idx_account_id")
            .await?
        {
            manager
                .create_index(
                    Index::create()
                        .if_not_exists()
                        .name("idx_account_id")
                        .table(DeboxGroup::Table)
                        .col(DeboxGroup::AccountId)
                        .to_owned(),
                )
                .await?;
        }

        // Sqlite 不支持外键
        if manager.get_database_backend() == DatabaseBackend::Sqlite {
            return Ok(());
        }

        if !manager
            .has_index(
                DeboxGroup::Table.to_string(),
                "fk_user_debox_group_account_id",
            )
            .await?
        {
            manager
                .create_foreign_key(
                    ForeignKey::create()
                        .name("fk_user_debox_group_account_id")
                        .from(DeboxGroup::Table, DeboxGroup::AccountId)
                        .to(DeboxAccount::Table, DeboxAccount::Id)
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
            .drop_table(Table::drop().table(DeboxGroup::Table).to_owned())
            .await
    }
}

// "https://m.debox.pro/group?id=l3izdfzd&code=2y9u8fkw",
#[derive(DeriveIden)]
pub enum DeboxGroup {
    #[sea_orm(iden = "t_debox_group")]
    Table,
    Id,
    AccountId,
    Url,
    GroupName,
    GroupCode,
    Desc,
    Status,
    CreatedAt,
    UpdatedAt,
}
