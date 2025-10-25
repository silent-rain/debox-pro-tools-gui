//! 用户相关表

pub mod email;
pub mod phone;
pub mod role;
pub mod user_base;
pub mod user_role_rel;

pub use email::Entity as EmailEntity;
pub use phone::Entity as PhoneEntity;
pub use user_base::Entity as UserBaseEntity;

pub use role::Entity as RoleEntity;
pub use user_role_rel::Entity as UserRoleRelEntity;
