//! tauri simple
use state::mobile::AppState;

use tauri::{Runtime, State, ipc::InvokeHandler};
use tokio::sync::Mutex;

/// 注册路由
pub fn tauri_register<R>() -> Box<InvokeHandler<R>>
where
    R: Runtime,
{
    Box::new(tauri::generate_handler![greet, increase_counter,])
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn increase_counter(state: State<'_, Mutex<AppState>>) -> Result<u32, ()> {
    let mut state = state.lock().await;
    state.counter += 1;
    Ok(state.counter)
}
