//! 移动端状态

use std::path::PathBuf;

use tracing_appender::non_blocking::WorkerGuard;

/// 应用目录
pub struct AppDirector {
    pub home_dir: PathBuf,
}

/// 全局应用状态
pub struct AppState {
    pub counter: u32,
    pub app_directory: AppDirector,

    pub log_guards: Vec<WorkerGuard>,
}
