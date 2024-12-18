use crate::commands::{calc_one_rm, calc_weights};
use std::sync::Arc;
use tauri::{Manager, Wry};
use tauri::path::BaseDirectory;
use tauri_plugin_store::{Store, StoreExt};

mod commands;
mod models;
mod tests;

pub struct AppState {
    store: Arc<Store<Wry>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let store = app.store("data.json")?;

            let resource_path = app.path().resolve("config/constants.toml", BaseDirectory::Resource)?;
            //TODO: load constants and add them to AppState

            app.manage(AppState { store });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![calc_weights, calc_one_rm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
