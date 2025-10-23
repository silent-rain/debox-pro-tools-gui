//! 应用目录

use std::path::PathBuf;

use log::info;
use tauri::{App, Manager};

/// 初始化系统目录
pub fn init_dir(app: &mut App) -> Result<PathBuf, tauri::Error> {
    let app_dir = if cfg!(target_os = "android") {
        let download_dir = app.path().download_dir()?;
        let app_dir = download_dir
            .parent()
            .ok_or_else(|| tauri::Error::NoParent)?
            .to_path_buf();
        if !app_dir.exists() {
            info!("========== create android app_dir: {:#?}", app_dir);
            std::fs::create_dir(&app_dir)?;
        }
        app_dir
    } else if cfg!(target_os = "linux") || cfg!(target_os = "windows") {
        let app_dir = app.path().app_config_dir()?;
        if !app_dir.exists() {
            info!("========== create desktop app_dir: {:#?}", app_dir);
            std::fs::create_dir(&app_dir)?;
        }
        app_dir
    } else if cfg!(debug_assertions) && !cfg!(target_os = "android") {
        // 调试模式下使用当前运行路径
        PathBuf::from("./")
    } else {
        // 其他场景, 使用当前运行路径
        PathBuf::from("./")
    };

    Ok(app_dir)
}

/// 打印系统目录
pub fn print_app_dir(app: &mut App) -> Result<(), tauri::Error> {
    /*
    Android:
    audio_dir: "/storage/emulated/0/Android/data/com.mobile_llm.app/files/Music"
    cache_dir: "/storage/emulated/0/Android/data/com.mobile_llm.app/cache"
    config_dir: "/data/user/0/com.mobile_llm.app"
    data_dir: "/data/user/0/com.mobile_llm.app"
    local_data_dir: "/data/user/0/com.mobile_llm.app"
    document_dir: "/storage/emulated/0/Android/data/com.mobile_llm.app/files/Documents"
    download_dir: "/storage/emulated/0/Android/data/com.mobile_llm.app/files/Download"
    picture_dir: "/storage/emulated/0/Android/data/com.mobile_llm.app/files/Pictures"
    public_dir: "/storage/emulated/0/Android/data/com.mobile_llm.app/files/DCIM"
    video_dir: "/storage/emulated/0/Android/data/com.mobile_llm.app/cache"
    resource_dir: "asset://localhost/"
    app_config_dir: "/data/user/0/com.mobile_llm.app"
    app_data_dir: "/data/user/0/com.mobile_llm.app"
    app_local_data_dir: "/data/user/0/com.mobile_llm.app"
    app_cache_dir: "/data/user/0/com.mobile_llm.app/cache"
    app_log_dir: "/data/user/0/com.mobile_llm.app/logs"
    temp_dir: "/data/user/0/com.mobile_llm.app/cache"
    home_dir: "/storage/emulated/0"

    Linux WSL2:
    audio_dir: UnknownPath
    cache_dir: "/home/one/.cache"
    config_dir: "/home/one/.config"
    data_dir: "/home/one/.local/share"
    local_data_dir: "/home/one/.local/share"
    document_dir: UnknownPath
    download_dir: UnknownPath
    picture_dir: UnknownPath
    public_dir: UnknownPath
    video_dir: UnknownPath
    resource_dir: "/home/one/code/mobile-llm/apps/target/x86_64-unknown-linux-gnu/debug"
    app_config_dir: "/home/one/.config/com.mobile-llm.app"
    app_data_dir: "/home/one/.local/share/com.mobile-llm.app"
    app_local_data_dir: "/home/one/.local/share/com.mobile-llm.app"
    app_cache_dir: "/home/one/.cache/com.mobile-llm.app"
    app_log_dir: "/home/one/.local/share/com.mobile-llm.app/logs"
    temp_dir: "/tmp"
    home_dir: "/home/one"
    */

    let document_dir = app.path().document_dir();
    info!("========== document_dir: {:#?}", document_dir);
    // if !Path::new(&document_dir).exists() {
    //     info!("==========create document_dir: {:#?}", document_dir);
    //     std::fs::create_dir(document_dir).unwrap();
    // }

    // Linux/macOS/Windows
    let audio_dir = app.path().audio_dir();
    info!("========== audio_dir: {:#?}", audio_dir);

    // Linux/macOS/Windows
    let cache_dir = app.path().cache_dir();
    info!("========== cache_dir: {:#?}", cache_dir);

    // Linux/macOS/Windows
    let config_dir = app.path().config_dir();
    info!("========== config_dir: {:#?}", config_dir);

    // Linux/macOS/Windows
    let data_dir = app.path().data_dir();
    info!("========== data_dir: {:#?}", data_dir);

    // Linux/macOS/Windows
    let local_data_dir = app.path().local_data_dir();
    info!("========== local_data_dir: {:#?}", local_data_dir);

    // Linux/macOS/Windows
    // 不支持 Android
    // let desktop_dir = app.path().desktop_dir();
    // info!("========== desktop_dir: {:#?}", desktop_dir);

    // Linux/macOS/Windows
    let document_dir = app.path().document_dir();
    info!("========== document_dir: {:#?}", document_dir);

    // Linux/macOS/Windows
    let download_dir = app.path().download_dir();
    info!("========== download_dir: {:#?}", download_dir);

    // Linux/macOS/Windows
    // let executable_dir = app.path().executable_dir();
    // info!("========== executable_dir: {:#?}", executable_dir);

    // Linux/macOS/Windows
    // let font_dir = app.path().font_dir();
    // info!("========== font_dir: {:#?}", font_dir);

    // Linux/macOS/Windows
    let picture_dir = app.path().picture_dir();
    info!("========== picture_dir: {:#?}", picture_dir);

    // Linux/macOS/Windows
    let public_dir = app.path().public_dir();
    info!("========== public_dir: {:#?}", public_dir);

    // Linux/macOS/Windows
    // let runtime_dir = app.path().runtime_dir();
    // info!("========== runtime_dir: {:#?}", runtime_dir);

    // Linux/Windows
    // **macOS:** Not supported.
    // 不支持 Android
    // let template_dir = app.path().template_dir();
    // info!("========== template_dir: {:#?}", template_dir);

    // Linux/macOS/Windows
    let video_dir = app.path().video_dir();
    info!("========== video_dir: {:#?}", video_dir);

    // Linux/macOS/Windows/IOS/Android
    let resource_dir = app.path().resource_dir();
    info!("========== resource_dir: {:#?}", resource_dir);

    let app_config_dir = app.path().app_config_dir()?;
    info!("========== app_config_dir: {:#?}", app_config_dir);

    let app_data_dir = app.path().app_data_dir();
    info!("========== app_data_dir: {:#?}", app_data_dir);

    let app_local_data_dir = app.path().app_local_data_dir();
    info!("========== app_local_data_dir: {:#?}", app_local_data_dir);

    let app_cache_dir = app.path().app_cache_dir();
    info!("========== app_cache_dir: {:#?}", app_cache_dir);

    let app_log_dir = app.path().app_log_dir();
    info!("========== app_log_dir: {:#?}", app_log_dir);

    let temp_dir = app.path().temp_dir();
    info!("========== temp_dir: {:#?}", temp_dir);

    // Linux/macOS/Windows
    // **iOS**: Cannot be written to directly, use one of the app paths instead.
    let home_dir = app.path().home_dir();
    info!("========== home_dir: {:#?}", home_dir);

    Ok(())
}
