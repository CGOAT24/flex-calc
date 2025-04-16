use crate::commands::{calc_one_rm, calc_weights, change_weight_unit};
use std::sync::Arc;
use tauri::{Manager, Wry};
use tauri::path::BaseDirectory;
use tauri_plugin_store::{Store, StoreExt};
use crate::models::config::Config;

mod commands;
mod models;

pub struct AppState {
    store: Arc<Store<Wry>>,
    constants: Config
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let resource_path = app.path().resolve("config/constants.json", BaseDirectory::Resource)?;
            let file = std::fs::File::open(&resource_path).unwrap();
            let config: Config = serde_json::from_reader(file).unwrap();

            let store = app.store(&config.store)?;

            app.manage(AppState { store, constants: config });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![calc_weights, calc_one_rm, change_weight_unit])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
