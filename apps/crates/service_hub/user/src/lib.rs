//! 用户信息管理
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::{
    phone::PhoneDao, role::RoleDao, user_base::UserBaseDao, user_role_rel::UserRoleRelDao,
};

pub(crate) mod service;
pub use service::{
    phone::PhoneService, role::RoleService, user_base::UserBaseService,
    user_role_rel::UserRoleRelService,
};

pub(crate) mod controller;
pub use controller::{
    phone::PhoneController, role::RoleController, user_base::UserBaseController,
    user_role_rel::UserRoleRelController,
};

pub(crate) mod router;
pub use router::{
    UserRouter, phone::PhoneRouter, role::RoleRouter, user_base::UserBaseRouter,
    user_role_rel::UserRoleRelRouter,
};
