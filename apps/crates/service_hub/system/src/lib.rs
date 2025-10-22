//! 系统管理
pub mod constant;
pub mod dto;
pub mod enums;

pub(crate) mod dao;
pub use dao::config::ConfigDao;

pub(crate) mod service;
pub use service::config::ConfigService;

pub(crate) mod controller;
pub use controller::config::ConfigController;

pub(crate) mod router;
pub use router::{SystemRouter, config::ConfigRouter};
