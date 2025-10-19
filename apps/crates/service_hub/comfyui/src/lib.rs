//! 模板管理
pub mod api_clients;

pub mod dto;

pub(crate) mod dao;

pub(crate) mod service;
pub use service::LocalService;

pub(crate) mod controller;
pub use controller::LocalController;

pub(crate) mod router;
