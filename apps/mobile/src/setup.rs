//! 初始化启动流程

use tauri::{App, Manager};
use tracing_appender::non_blocking::WorkerGuard;

use err_code::Error;
use logger::config::ConsoleBunyanConfig;
use state::mobile::{AppDirector, AppState};

use crate::utils::app_dir::{init_dir, print_app_dir};

pub struct Setup {}

impl Setup {
    pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化应用目录
        let app_dir = init_dir(app).expect("初始化应用目录失败");

        // 初始化日志
        let log_dir = app_dir.join("logs").to_string_lossy().to_string();
        let log_guards = Self::init_logger(&log_dir).expect("初始化日志失败");

        let state = AppState {
            counter: 0,
            app_directory: AppDirector {
                app_dir,
                home_dir: "".into(),
            },
            log_guards,
        };
        app.manage(state);

        // 打印系统目录
        print_app_dir(app).expect("打印系统目录失败");

        Ok(())
    }

    /// 初始化日志
    pub fn init_logger(log_dir: &str) -> Result<Vec<WorkerGuard>, Error> {
        let logger = logger::config::LoggerConfig {
            file: logger::config::FileConfig {
                filepath: log_dir.to_string(),
                level: logger::config::Level::Info,
                enable: true,
                ..Default::default()
            },
            console_bunyan: ConsoleBunyanConfig {
                level: logger::config::Level::Info,
                enable: true,
            },
            ..Default::default()
        };
        let guards = logger::Logger::build(&logger).expect("初始化日志失败");

        Ok(guards)
    }
}
