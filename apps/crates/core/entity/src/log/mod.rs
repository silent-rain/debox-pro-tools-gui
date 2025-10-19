//! 日志相关表

pub mod log_api_operation;
pub mod log_system;
pub mod log_web;

pub use log_api_operation::Entity as LogApiOperationEntity;
pub use log_system::Entity as LogSystemEntity;
pub use log_web::Entity as LogWebEntity;
