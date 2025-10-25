//! 工具集
#![allow(unused)]
use sea_orm::{
    Iden,
    sea_query::{Index, IntoIden, IntoIndexColumn, IntoTableRef},
};
use sea_orm_migration::{DbErr, SchemaManager};

/// 设置索引
///
/// 如果不存在则设置索引
pub async fn if_not_exists_create_index<T, C>(
    manager: &SchemaManager<'_>,
    table: T,
    cols: Vec<C>,
) -> Result<(), DbErr>
where
    T: IntoTableRef + Iden,
    C: IntoIndexColumn + Iden + IntoIden,
{
    if cols.is_empty() {
        return Ok(());
    }

    let mut names = vec!["idx".to_string(), table.to_string()];
    for col in &cols {
        names.push(col.to_string());
    }
    let name_str = names.join("_");

    let mut stmt = Index::create();
    stmt.if_not_exists().name(name_str).table(table);
    for col in cols.into_iter() {
        stmt.col(col);
    }

    manager.create_index(stmt.to_owned()).await?;

    Ok(())
}

/// 设置联合索引
///
/// 如果不存在则设置索引
pub async fn if_not_exists_create_unique_index<T, C>(
    manager: &SchemaManager<'_>,
    table: T,
    cols: Vec<C>,
) -> Result<(), DbErr>
where
    T: IntoTableRef + Iden,
    C: IntoIndexColumn + Iden + IntoIden,
{
    if cols.is_empty() {
        return Ok(());
    }

    let mut names = vec!["uni_".to_string(), table.to_string()];
    for col in &cols {
        names.push(col.to_string());
    }
    let name_str = names.join("_");

    let mut stmt = Index::create();
    stmt.if_not_exists().name(name_str).table(table).unique();
    for col in cols.into_iter() {
        stmt.col(col);
    }

    manager.create_index(stmt.to_owned()).await?;

    Ok(())
}
