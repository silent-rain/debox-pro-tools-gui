//! 初始化启动流程

use std::{path::Path, sync::Arc};

use log::info;
use tauri::{App, Manager, path::BaseDirectory};
use tracing_appender::non_blocking::WorkerGuard;

use admin::server::HttpServer;
use app_state::mobile::{AppDirector, AppState};
use config::AppConfig;
use database::Mdb;
use err_code::Error;
use inject::InjectProvider;

use crate::utils::app_dir::{init_dir, print_app_dir};

const CONFIG_FILE: &str = "config.yaml";
const DATA_DAT_FILE: &str = "data.dat";

pub struct Setup {}

impl Setup {
    pub fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
        // 初始化应用目录
        let app_dir = init_dir(app).expect("初始化应用目录失败");

        // 初始化资源文件
        Self::init_resources(app, &app_dir).expect("初始化资源文件失败");

        // 加载配置文件
        let app_config = AppConfig::new(app_dir.join(CONFIG_FILE).to_string_lossy().as_ref())
            .expect("加载配置文件失败");

        // 初始化日志
        let log_guards = Self::init_logger(&app_dir, &app_config).expect("初始化日志失败");

        // 加载数据库
        let db_pool = Self::load_data_dat(&app_dir, &app_config);

        // Using an Arc to share the provider across multiple threads.
        let inject_provider = Arc::new(InjectProvider::new(db_pool.clone()));

        // 全局状态
        let state = Arc::new(AppState {
            counter: 0,
            log_guards,
            app_directory: AppDirector {
                app_dir,
                home_dir: "".into(),
            },
        });
        app.manage(state.clone());

        // 打印系统目录
        print_app_dir(app).expect("打印系统目录失败");

        // 启动 Http 服务
        let state_c = state.clone();
        tauri::async_runtime::spawn(async {
            HttpServer::run(app_config, db_pool, inject_provider, state_c)
                .await
                .expect("初始化接口服务失败")
        });

        Ok(())
    }

    /// 初始化日志
    pub fn init_logger(app_dir: &Path, app_config: &AppConfig) -> Result<Vec<WorkerGuard>, Error> {
        let log_dir = app_dir.join("logs").to_string_lossy().to_string();

        let mut logger = app_config.logger.clone();
        logger.file.filepath = log_dir.to_string();

        let guards = logger::Logger::build(&logger).expect("初始化日志失败");

        Ok(guards)
    }

    /// 初始化资源
    ///
    /// 读取 Resources 目录中文件
    pub fn init_resources(app: &mut App, app_dir: &Path) -> Result<(), Error> {
        // 配置文件
        let config_path = app.path().resolve(CONFIG_FILE, BaseDirectory::Resource)?;
        // 数据库配置
        let db_path = app.path().resolve(DATA_DAT_FILE, BaseDirectory::Resource)?;

        if cfg!(target_os = "android")
            || (!cfg!(debug_assertions) && cfg!(target_os = "linux"))
            || (!cfg!(debug_assertions) && cfg!(target_os = "windows"))
        {
            if !config_path.exists() {
                std::fs::copy(&config_path, app_dir.join(CONFIG_FILE))?;

                info!(
                    "copy resource config: {:#?} to {:#?}",
                    config_path,
                    app_dir.join(CONFIG_FILE)
                );
            }

            if !db_path.exists() {
                std::fs::copy(&db_path, app_dir.join(DATA_DAT_FILE))?;

                info!(
                    "copy resource config: {:#?} to {:#?}",
                    db_path,
                    app_dir.join(DATA_DAT_FILE)
                );
            }
        }
        Ok(())
    }

    /// 加载数据库
    ///
    /// 加载 data.dat 数据库文件
    pub fn load_data_dat(app_dir: &Path, app_config: &AppConfig) -> Mdb {
        tauri::async_runtime::block_on(async {
            let mut sqlite = app_config.sqlite.clone();
            sqlite.sqlite_path = Some(format!(
                "sqlite://{}?mode=rwc",
                app_dir.join(DATA_DAT_FILE).to_string_lossy()
            ));

            // 初始化数据库
            let main_db = database::Pool::new(sqlite.dns(), sqlite.options.clone())
                .await
                .expect("初始化数据库失败");

            Mdb::new(Arc::new(main_db.clone()), Arc::new(main_db))
        })
    }
}
