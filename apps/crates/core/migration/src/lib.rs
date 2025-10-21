use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

mod m20230617_084425_create_log_system;
mod m20240218_145453_create_user_base;
mod m20240218_161916_create_sys_config;
mod m20251020_145453_create_debox_account;
mod m20251020_145453_create_debox_group;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 用户管理
            Box::new(m20240218_145453_create_user_base::Migration),
            // DeBox 管理
            Box::new(m20251020_145453_create_debox_account::Migration),
            Box::new(m20251020_145453_create_debox_group::Migration),
            // 系统管理
            Box::new(m20240218_161916_create_sys_config::Migration),
            // 日志管理
            Box::new(m20230617_084425_create_log_system::Migration),
        ]
    }
}
