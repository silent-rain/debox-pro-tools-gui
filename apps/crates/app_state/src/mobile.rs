//! 移动端状态

use std::path::PathBuf;

use tracing_appender::non_blocking::WorkerGuard;

/// 应用目录
#[derive(Debug, Default, Clone)]
pub struct AppDirector {
    pub home_dir: PathBuf,
    pub app_dir: PathBuf,
}

/// 全局应用状态
#[derive(Debug, Default)]
pub struct AppState {
    pub counter: u32,
    pub app_directory: AppDirector,

    pub log_guards: Vec<WorkerGuard>,
}
