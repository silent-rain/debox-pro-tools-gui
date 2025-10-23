mod config;
mod dto;
mod middleware;
mod router;
mod server;
mod setup;
mod utils;

use router::tauri_router;
use setup::Setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_level(true)
    //     .with_line_number(true)
    //     .init();

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
    // tokio::spawn(async {
    //     HttpServer::run(app_config, db_pool, inject_provider)
    //         .await
    //         .expect("初始化接口服务失败");
    // });

    // tauri::async_runtime::spawn(async {
    //     HttpServer::run(app_config, db_pool, inject_provider)
    //         .await
    //         .expect("初始化接口服务失败")
    // });

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
