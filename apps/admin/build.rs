use std::{env, fs};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 拷贝 `apps/config.yaml` 文件到当前目录
    let config_file = "../config.yaml";
    let current_dir = env::current_dir()?;
    let target_file = current_dir.join("config.yaml");
    fs::copy(config_file, target_file)?;

    Ok(())
}
