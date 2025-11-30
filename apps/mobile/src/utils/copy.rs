//! 文件拷贝

use std::path::Path;

use log::{error, info};
use tauri::App;
use tauri_plugin_fs::FsExt;

use err_code::Error;

pub fn copy_file(app: &mut App, src: &Path, dst: &Path) -> Result<(), Error> {
    let content = app.fs().read(src).map_err(|e| {
        error!("read {src:#?} failed: {e}");
        println!("read {src:#?} failed: {e}");
        e
    })?;

    std::fs::write(dst, content).map_err(|e| {
        error!("copy file from {src:#?} to {dst:#?} failed: {e}");
        println!("copy file from {src:#?} to {dst:#?} failed: {e}");
        e
    })?;
    info!("copy file from {src:#?} to {dst:#?} success");

    Ok(())
}
