use std::env;

mod common;
use common::{User, schem::create_user_table, user};

use database::{Curd, Options, Pagination, Pool, PoolTrait};

use sea_orm::{DbErr, EntityTrait, Set};

async fn testcase(db: &Pool) -> Result<(), DbErr> {
    let active_model = user::ActiveModel {
        name: Set("zhangshan".to_string()),
        status: Set(true),
        ..Default::default()
    };

    let result = User::insert(active_model)
        .exec(db.db())
        .await
        .expect("could not insert baker");

    assert_eq!(result.last_insert_id, 1);

    Ok(())
}

struct UserDao<'a> {
    db: &'a Pool,
}

impl Curd<user::Entity> for UserDao<'_> {
    type Db = Pool;

    fn db(&self) -> &Self::Db {
        self.db
    }
}

async fn test_all(db: &Pool) -> Result<(Vec<user::Model>, u64), DbErr> {
    let dao = UserDao { db };
    let (results, total) = dao.all().await.unwrap();
    println!("test_all ======= {:#?}", results);
    Ok((results, total))
}

async fn test_list(db: &Pool) -> Result<(Vec<user::Model>, u64), DbErr> {
    let dao = UserDao { db };
    let page = Pagination::new(0, 10);
    let (results, total) = dao.list(page).await.unwrap();
    println!("test_list ======= {:#?}", results);
    Ok((results, total))
}

// async fn list_pages(db: &Pool) -> Result<(Vec<user::Model>, u64), DbErr> {
//     let dao = UserDao { db };
//     let page = Pagination::new(0, 10);
//     let (results, total) = dao.list_pages(page).await.unwrap();
//     println!("list_pages ======= {:#?}", results);
//     Ok((results, total))
// }

async fn test_info(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let result = dao.info(1).await.unwrap().unwrap();
    println!("test_info ======= {:#?}", result);
    Ok(result)
}

async fn test_insert(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::ActiveModel {
        name: Set("zhangshan".to_string()),
        status: Set(true),
        ..Default::default()
    };
    let result = dao.insert(data).await.unwrap();
    println!("test_insert ======= {:#?}", result);
    Ok(result)
}

async fn test_insert2(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::Model {
        name: "zhangshan".to_string(),
        status: true,
        ..Default::default()
    };
    let result = dao
        ._insert2::<user::Model, user::ActiveModel>(data)
        .await
        .unwrap();
    println!("test_insert2 ======= {:#?}", result);
    Ok(result)
}

async fn test_update(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::ActiveModel {
        id: Set(2),
        name: Set("zhangshan".to_string()),
        status: Set(false),
    };
    let result = dao.update(data).await.unwrap();
    println!("test_update ======= {:#?}", result);
    Ok(result)
}

async fn test_update2(db: &Pool) -> Result<user::Model, DbErr> {
    let dao = UserDao { db };
    let data = user::Model {
        id: 1,
        name: "zhangshan".to_string(),
        status: false,
    };
    let result = dao
        ._update2::<user::Model, user::ActiveModel>(data)
        .await
        .unwrap();
    println!("test_update2 ======= {:#?}", result);
    Ok(result)
}

async fn test_delete(db: &Pool) -> Result<u64, DbErr> {
    let dao = UserDao { db };

    let result = dao.delete(0).await.unwrap();
    println!("test_delete ======= {:#?}", result);
    Ok(result)
}

async fn test_batch_delete(db: &Pool) -> Result<u64, DbErr> {
    let dao = UserDao { db };

    let result = dao.batch_delete(vec![1, 2]).await.unwrap();
    println!("test_batch_delete ======= {:#?}", result);
    Ok(result)
}

// cargo test --package database --test sqlite_test -- --nocapture
#[tokio::test]
async fn main() -> Result<(), DbErr> {
    // TODO: Audit that the environment access only happens in single-threaded code.
    unsafe { env::set_var("RUST_BACKTRACE", "1") };

    tracing_subscriber::fmt()
        .compact()
        .with_max_level(tracing::Level::INFO)
        .with_level(true)
        .with_line_number(true)
        .init();

    // Connecting SQLite
    let db_url = "sqlite::memory:".to_string();
    let db = Pool::connect(db_url, Options::default())
        .await
        .expect("db init failed");
    let pool = Pool::form_connect(db);

    // Setup database schema
    create_user_table(&pool).await?;

    // Performing tests
    testcase(&pool).await?;
    test_info(&pool).await?;

    test_insert(&pool).await?;
    test_insert2(&pool).await?;

    test_list(&pool).await?;

    test_update(&pool).await?;
    test_update2(&pool).await?;

    test_list(&pool).await?;

    test_delete(&pool).await?;

    test_all(&pool).await?;

    test_batch_delete(&pool).await?;

    test_list(&pool).await?;

    Ok(())
}
