mod dto;
mod middleware;
mod router;
mod server;
mod setup;
mod utils;

use server::HttpServer;

use setup::Setup;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // tracing_subscriber::fmt()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .with_level(true)
    //     .with_line_number(true)
    //     .init();

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .setup(Setup::setup)
        .invoke_handler(router::tauri_register());

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .enable_io()
        .enable_time()
        .build()
        .expect("初始化运行时失败");
    runtime.spawn(async {
        HttpServer::run().await.expect("初始化接口服务失败");
    });

    // runtime.spawn(async {
    //     let pipeline = simple_llama_pipeline().await.unwrap();
    //     PIPELINE.get_or_init(|| pipeline);
    //     warn!("model laod success ...");
    // });

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
