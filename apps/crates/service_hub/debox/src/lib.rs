//! Debox管理
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::{
    debox_account::DeboxAccountDao, debox_account_follow::DeboxAccountFollowDao,
    debox_group::DeboxGroupDao, debox_group_member::DeboxGroupMemberDao,
};

pub(crate) mod service;
pub use service::{
    debox_account::DeboxAccountService, debox_account_follow::DeboxAccountFollowService,
    debox_group::DeboxGroupService, debox_group_member::DeboxGroupMemberService,
};

pub(crate) mod controller;
pub use controller::{
    debox_account::DeboxAccountController, debox_account_follow::DeboxAccountFollowController,
    debox_group::DeboxGroupController, debox_group_member::DeboxGroupMemberController,
};

pub(crate) mod router;
pub use router::{
    DeboxRouter, debox_account::DeboxAccountRouter, debox_group::DeboxGroupRouter,
    debox_group_member::DeboxGroupMemberRouter,
};
