use sea_orm_migration::{MigrationTrait, async_trait};

mod debox;
pub use migration_git::*;
pub use sea_orm_migration::migrator::MigratorTrait;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // 用户管理
            Box::new(user::user_base::Migration),
            Box::new(user::user_phone::Migration),
            Box::new(user::user_email::Migration),
            Box::new(user::role::Migration),
            Box::new(user::user_role_rel::Migration),
            // DeBox 管理
            Box::new(debox::debox_account::Migration),
            Box::new(debox::debox_group::Migration),
            // 系统管理
            Box::new(system::config::Migration),
            Box::new(system::dict_dimension::Migration),
            Box::new(system::dict_data::Migration),
            Box::new(system::image_captcha::Migration),
            Box::new(system::file_resource::Migration),
            // 日志管理
            Box::new(log::log_api_operation::Migration),
            Box::new(log::log_system::Migration),
            Box::new(log::log_web::Migration),
        ]
    }
}
