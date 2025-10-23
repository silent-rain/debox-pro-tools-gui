mod config;
mod dto;
mod middleware;
mod router;
mod server;
mod setup;
mod utils;

use std::sync::Arc;

use database::Mdb;
use inject::InjectProvider;
use server::HttpServer;

use config::AppConfig;
use router::tauri_router;
use setup::Setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_level(true)
    //     .with_line_number(true)
    //     .init();

    // 加载配置文件
    let app_config = AppConfig::new("config.yaml").expect("加载配置文件失败");

    // 初始化数据库
    let main_db = database::Pool::new(app_config.sqlite.dns(), app_config.sqlite.options.clone())
        .await
        .expect("初始化数据库失败");
    let db_pool = Mdb::new(Arc::new(main_db.clone()), Arc::new(main_db));

    // 全局状态
    // let state = Arc::new(AppState {});

    // Using an Arc to share the provider across multiple threads.
    let inject_provider = Arc::new(InjectProvider::new(db_pool.clone()));

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(Setup::setup)
        .invoke_handler(tauri_router::register());

    // let runtime = tokio::runtime::Builder::new_multi_thread()
    //     .enable_all()
    //     .enable_io()
    //     .enable_time()
    //     .build()
    //     .expect("初始化运行时失败");
    // runtime.spawn(async {
    //     HttpServer::run(app_config, db_pool, inject_provider)
    //         .await
    //         .expect("初始化接口服务失败");
    // });

    // 启动 Http 服务
    tokio::spawn(async {
        HttpServer::run(app_config, db_pool, inject_provider)
            .await
            .expect("初始化接口服务失败");
    });

    // use tauri::async_runtime::block_on( || {});

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
