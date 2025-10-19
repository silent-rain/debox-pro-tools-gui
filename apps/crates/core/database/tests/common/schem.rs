#![allow(unused)]
use database::{Pool, PoolTrait};

use sea_orm::{
    ConnectionTrait, DbBackend, DbErr, EntityTrait, ExecResult, Schema,
    sea_query::{ColumnDef, Table, TableCreateStatement},
};

use super::User;
use super::user;

pub async fn create_user_table(db: &Pool) -> Result<ExecResult, DbErr> {
    let stmt = Table::create()
        .table(User)
        .col(
            ColumnDef::new(user::Column::Id)
                .integer()
                .not_null()
                .auto_increment()
                .primary_key()
                .comment("ID"),
        )
        .col(
            ColumnDef::new(user::Column::Name)
                .string()
                .not_null()
                .comment("用户名称"),
        )
        .col(
            ColumnDef::new(user::Column::Status)
                .boolean()
                .not_null()
                .default(true)
                .comment("状态,false:禁用,true:启用"),
        )
        .to_owned();

    create_table(db, &stmt, User).await
}

pub async fn create_table<E>(
    db: &Pool,
    create: &TableCreateStatement,
    entity: E,
) -> Result<ExecResult, DbErr>
where
    E: EntityTrait,
{
    let builder = db.db().get_database_backend();
    let schema = Schema::new(builder);
    assert_eq!(
        builder.build(&schema.create_table_from_entity(entity)),
        builder.build(create)
    );

    create_table_without_asserts(db, create).await
}

pub async fn create_table_without_asserts(
    db: &Pool,
    create: &TableCreateStatement,
) -> Result<ExecResult, DbErr> {
    let builder = db.db().get_database_backend();
    if builder != DbBackend::Sqlite {
        let stmt = builder.build(
            Table::drop()
                .table(create.get_table_name().unwrap().clone())
                .if_exists()
                .cascade(),
        );
        db.db().execute(stmt).await?;
    }
    db.db().execute(builder.build(create)).await
}
